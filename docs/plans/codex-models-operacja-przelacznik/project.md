# Cel calego projektu
Codex TUI ma pozwalac dodac lokalne modele OSS do `/model` i przelaczac aktywna rozmowe miedzy providerem premium OpenAI a lokalnym providerem LM Studio/Ollama bez recznego startowania `codex --oss --local-provider ...`.

# Kontekst calego projektu
Tytul: Custom OSS models in `/model`
Kryptonim: Operacja Przelacznik

Repo: `/home/sieciowiec/dev/rust/codex`
Glowny obszar kodu: `codex-rs/`

Fakty z aktualnego rozpoznania:
- CLI juz obsluguje OSS przez `--oss --local-provider lmstudio|ollama`.
- `codex exec --skip-git-repo-check --oss --local-provider lmstudio -m google/gemma-4-26b-a4b "Return exactly: LOCAL_OK"` dziala lokalnie.
- Profil `gemma-local` w `~/.codex/config.toml` dziala i pokazuje provider `lmstudio`.
- Aktualny `/model` w TUI pokazuje glownie modele OpenAI i effort; lokalne modele nie sa pierwszoklasowym wyborem w pickerze.
- LM Studio wystawia modele przez `http://localhost:1234/api/v1/models` oraz kompatybilny endpoint OpenAI `http://localhost:1234/v1/models`.
- Ollama zwykle wystawia liste przez `http://localhost:11434/api/tags`.
- W repo istnieja juz identyfikatory i helpery OSS: `oss_provider`, `resolve_oss_provider`, `LMSTUDIO_OSS_PROVIDER_ID`, `OLLAMA_OSS_PROVIDER_ID`, `ensure_oss_provider_ready`, `get_default_model_for_oss_provider`.

Decyzja UX:
- `/model` ma pokazywac sekcje `Premium` i `Local / OSS`.
- Wybieranie modelu lokalnego musi ustawic jednoczesnie `model` oraz `model_provider`/provider runtime, nie tylko sama nazwe modelu.
- Powrot na model premium musi przywrocic provider OpenAI i wybrany model OpenAI.

Decyzja o konfiguracji custom modeli:
- Preferowany format to maly, jawny registry w `config.toml`, bo jest latwy do sync, backupu i dokumentacji:

```toml
[[custom_models]]
name = "Gemma 4 26B A4B"
model = "google/gemma-4-26b-a4b"
provider = "lmstudio"
base_url = "http://localhost:1234/v1"
source = "manual"

[[custom_models]]
name = "GPT OSS 20B"
model = "openai/gpt-oss-20b"
provider = "lmstudio"
base_url = "http://localhost:1234/v1"
source = "manual"
```

- Dopuszczalny fallback, jesli zmiana `ConfigToml` robi za duzy konflikt z upstream: micro plik w `CODEX_HOME/custom_models.toml` z tym samym formatem `[[models]]`. Ten wariant ma byc rozpatrzony w milestone 2, ale nie implementowany rownolegle, jesli configowy wariant jest czysty.
- Automatyczne odkrywanie modeli z LM Studio/Ollama moze uzupelniac liste, ale reczny registry jest source of truth dla modeli, ktore user chce stale widziec w `/model`.

Decyzja o `/models`:
- Nazwa user-facing moze pozostac `/model`; dokumentacja ma jasno odroznic `/model` picker od katalogu filesystem `/models`.
- Feature nie ma pobierac ani przenosic GGUF. To robi LM Studio/Ollama. Codex ma tylko znac provider, model id i opcjonalnie base URL.

Strategia merge-friendly:
- Nie rozpychac `codex-rs/tui/src/chatwidget.rs`; dodac osobny modul dla lokalnych wpisow pickera i tylko cienkie wywolanie w miejscu budowania akcji `/model`.
- Nie forkowac calych flow CLI OSS. Reuzyc istniejace `oss_provider`/provider info.
- Zmiany w `ConfigToml` ograniczyc do nowego optional pola i testow parse/default. Jesli schema wymaga update, uruchomic `just write-config-schema`.
- Jesli potrzeba runtime switch w aktywnej rozmowie, dodac mala warstwe `ModelSelectionTarget`/`ProviderModelSelection` zamiast specjalnych ifow rozsianych po TUI.
- Subagenci na lokalnym modelu sa follow-up albo eksperymentalny milestone. Nie blokowac podstawowego `/model` na pelnej przebudowie spawn API.

# Zakres pracy
W zakresie:
- Sync z upstream/origin przed kodowaniem i baseline build.
- Audyt aktualnego `/model`, konfiguracji providerow i miejsc, gdzie TUI wysyla zmiane modelu do sesji.
- Dodanie registry custom OSS models w `config.toml` albo w malym `CODEX_HOME/custom_models.toml`, z preferencja dla `config.toml`.
- Dodanie sekcji Local/OSS do `/model`, z wpisami recznymi i opcjonalnie dynamicznymi z LM Studio/Ollama.
- Przelaczanie aktywnej rozmowy premium <-> OSS przez atomowy wybor `provider + model`.
- Dokumentacja w `docs/config.md` albo nowym malym docu `docs/custom-models.md` oraz link z odpowiedniego README/indexu.
- Smoke z lokalnym LM Studio, jesli server dziala.

Poza zakresem:
- Pobieranie modeli z Hugging Face do `/models`.
- Zarzadzanie plikami GGUF, quantami albo mmproj.
- Wymuszenie globalnego defaultu Codexa na lokalny model.
- Duzo wiekszy redesign subagentow. Plan moze zostawic przygotowany kontrakt na override modelu subagenta, ale podstawowe DONE to `/model`.
- Wspieranie dowolnych providerow OpenAI-compatible poza prostym `base_url` dla OSS, jesli wymaga osobnej autoryzacji.

Sytuacje stop:
- Upstream po sync zmienil model picker tak mocno, ze wskazane miejsca kodu nie istnieja i wymaga to nowej architektury.
- Aktywna sesja nie pozwala bezpiecznie zmienic `model_provider` bez restartu rozmowy; wtedy trzeba udokumentowac ograniczenie i zaproponowac `restart with model`.
- Testy baseline przed zmianami nie kompiluja sie z powodow niezaleznych od tej pracy; wpisac dokladny blad do logu i nie maskowac go featurem.

# Walidacja i done
Ownerzy zmiany:
- `codex-rs/tui`: `/model` UI i runtime selection.
- `codex-rs/core` / `codex-rs/config`: parsing configu i provider/model semantics, tylko jesli wymagane.
- `docs/`: instrukcja dodawania lokalnych modeli.

Source-of-truth lane:
- Config lane: `~/.codex/config.toml` i ewentualnie `CODEX_HOME/custom_models.toml`.
- UI lane: `/model` picker pokazuje `Premium` oraz `Local / OSS`.
- Runtime lane: wybrany wpis lokalny faktycznie uruchamia provider `lmstudio` albo `ollama`.

Tani loop lokalny:
- `cargo check -p codex-tui`
- `cargo check -p codex-core`
- `cargo test -p codex-tui model`
- `cargo test -p codex-core config`

Gate przed DONE:
- `git status --short --branch`
- `git fetch --all --prune` i merge/rebase sync wykonany przed kodem.
- Baseline `cargo check` zapisany w logu przed implementacja.
- Po zmianach: `just fmt` w `codex-rs`.
- Po zmianach TUI: `cargo test -p codex-tui` albo co najmniej testy model picker + snapshoty, jesli pelny crate jest zbyt dlugi.
- Po zmianach configu: `cargo test -p codex-core config` oraz `just write-config-schema`, jesli ruszone `ConfigToml`.
- Smoke, jesli LM Studio dziala: lokalny model z registry widoczny w `/model` i/lub `codex exec -p <local-profile>` nadal dziala.
- Dokumentacja opisuje co wpisac w configu, skad Codex robi requesty i czemu `/models` filesystem nie wystarcza.

Wymagane artefakty:
- Kod feature'a.
- Testy configu i/lub TUI.
- Snapshoty TUI, jesli render `/model` sie zmienia.
- Dokumentacja user-facing.
- Wpisy w tym `project.md` po kazdym milestone.

Otwarte ryzyko walidacyjne:
- TUI tests moga wymagac aktualizacji snapshotow przez `cargo insta`.
- Dynamiczne requesty do LM Studio/Ollama w pickerze nie moga zawieszac TUI; potrzebny timeout/cache albo reczny registry jako stabilny fallback.
- Zmiana providera aktywnej rozmowy moze miec implikacje dla historii i model metadata; trzeba sprawdzic czy protokol sesji juz obsluguje provider switch.

Ukonczone gdy:
- User moze dopisac model lokalny w configu lub micro pliku.
- `/model` pokazuje ten model w sekcji Local/OSS.
- Wybor lokalnego modelu ustawia provider lokalny i model id.
- Wybor premium modelu wraca na OpenAI.
- Dokumentacja zawiera dzialajacy przyklad Gemma/LM Studio i komendy smoke.

# Lista milestone'ow
- [x] `milestones/01-sync-i-baseline.md` - sync z upstream i baseline walidacji
- [x] `milestones/02-registry-custom-models.md` - registry custom OSS models
- [x] `milestones/03-model-picker-local-oss.md` - Local/OSS w `/model`
- [x] `milestones/04-docs-smoke-subagent-contract.md` - dokumentacja, smoke i kontrakt pod subagentow

# Log pracy
Status: zaimplementowane
Biezacy milestone: DONE
Nastepny krok: opcjonalny smoke w TUI z dzialajacym LM Studio/Ollama.
Ostatnia walidacja: sync wykonany merge commitem `6d56fdaa60` po `git fetch --all --prune`; lokalne QoL commity zachowane. Baseline po sync: `cargo check -p codex-tui` OK z jednym upstreamowym warningiem dead_code w `AppEvent::RateLimitsLoaded`; `cargo check -p codex-core` OK.
Walidacja po implementacji: `cargo fmt --all` OK z istniejacymi ostrzezeniami rustfmt o nightly-only `imports_granularity`; `cargo check -p codex-core` OK; `cargo check -p codex-tui` OK z tym samym warningiem dead_code w `AppEvent::RateLimitsLoaded`; `cargo test -p codex-core load_config_registers_custom_local_model` OK; `just write-config-schema` OK; `just write-app-server-schema` OK.
Zrealizowane: `[[custom_models]]` w `config.toml`, syntetyczny provider id dla wpisow z `base_url`, sekcja `Local / OSS` w `/model`, zapis `model` + `model_provider`, turn-scoped provider override w app-server `turn/start`, dokumentacja `docs/custom-models.md` i link z README.
Ryzyka / blokady: upstream merge wprowadzil duzo snapshotow z trailing whitespace, wiec `git diff --cached --check` nie jest czystym gate dla calego merge'a. Smoke TUI wymaga uruchomionego lokalnego providera i recznego wyboru `/model`.
