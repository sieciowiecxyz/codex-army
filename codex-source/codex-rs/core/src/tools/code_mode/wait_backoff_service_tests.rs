use super::CodeModeService;
use codex_code_mode::CodeModeSession;
use codex_code_mode::CodeModeSessionDelegate;
use codex_code_mode::CodeModeSessionProvider;
use codex_code_mode::CodeModeSessionProviderFuture;
use codex_code_mode::CodeModeSessionResultFuture;
use codex_code_mode::ExecuteRequest;
use codex_code_mode::RuntimeResponse;
use codex_code_mode::StartedCell;
use codex_code_mode::WaitOutcome;
use codex_code_mode::WaitRequest;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

struct RecordingSession {
    outcomes: Mutex<VecDeque<WaitOutcome>>,
    requests: Mutex<Vec<WaitRequest>>,
}

impl RecordingSession {
    fn new(outcomes: impl IntoIterator<Item = WaitOutcome>) -> Self {
        Self {
            outcomes: Mutex::new(outcomes.into_iter().collect()),
            requests: Mutex::new(Vec::new()),
        }
    }
}

impl CodeModeSession for RecordingSession {
    fn execute<'a>(
        &'a self,
        _request: ExecuteRequest,
    ) -> CodeModeSessionResultFuture<'a, StartedCell> {
        Box::pin(async { Err("execute is not used by this test".to_string()) })
    }

    fn wait<'a>(&'a self, request: WaitRequest) -> CodeModeSessionResultFuture<'a, WaitOutcome> {
        Box::pin(async move {
            self.requests.lock().await.push(request);
            self.outcomes
                .lock()
                .await
                .pop_front()
                .ok_or_else(|| "test session ran out of outcomes".to_string())
        })
    }

    fn terminate<'a>(
        &'a self,
        cell_id: codex_code_mode::CellId,
    ) -> CodeModeSessionResultFuture<'a, WaitOutcome> {
        Box::pin(async move {
            Ok(WaitOutcome::LiveCell(RuntimeResponse::Terminated {
                cell_id,
                content_items: Vec::new(),
            }))
        })
    }

    fn shutdown<'a>(&'a self) -> CodeModeSessionResultFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }
}

struct RecordingProvider {
    session: Arc<RecordingSession>,
}

impl CodeModeSessionProvider for RecordingProvider {
    fn create_session<'a>(
        &'a self,
        _delegate: Arc<dyn CodeModeSessionDelegate>,
    ) -> CodeModeSessionProviderFuture<'a> {
        let session = Arc::clone(&self.session);
        Box::pin(async move { Ok(session as Arc<dyn CodeModeSession>) })
    }
}

#[tokio::test]
async fn short_empty_waits_are_coalesced_with_adaptive_backoff() {
    let cell_id = codex_code_mode::CellId::new("cell-1".to_string());
    let session = Arc::new(RecordingSession::new([
        WaitOutcome::LiveCell(RuntimeResponse::Yielded {
            cell_id: cell_id.clone(),
            content_items: Vec::new(),
        }),
        WaitOutcome::LiveCell(RuntimeResponse::Yielded {
            cell_id: cell_id.clone(),
            content_items: Vec::new(),
        }),
        WaitOutcome::LiveCell(RuntimeResponse::Result {
            cell_id: cell_id.clone(),
            content_items: vec![codex_code_mode::FunctionCallOutputContentItem::InputText {
                text: "done".to_string(),
            }],
            error_text: None,
        }),
    ]));
    let service = CodeModeService::new(
        Arc::new(RecordingProvider {
            session: Arc::clone(&session),
        }),
        &Default::default(),
        None,
    );

    let outcome = service
        .wait(WaitRequest {
            cell_id,
            yield_time_ms: 1_000,
        })
        .await
        .unwrap();

    assert!(matches!(
        outcome,
        WaitOutcome::LiveCell(RuntimeResponse::Result { .. })
    ));
    assert_eq!(
        session
            .requests
            .lock()
            .await
            .iter()
            .map(|request| request.yield_time_ms)
            .collect::<Vec<_>>(),
        vec![1_000, 2_000, 4_000]
    );
}
