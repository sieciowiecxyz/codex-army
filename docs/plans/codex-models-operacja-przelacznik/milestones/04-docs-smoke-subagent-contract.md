# Cel milestone'u - dokumentacja, smoke i kontrakt pod subagentow
Feature ma user-facing instrukcje dodawania lokalnych modeli do `/model`, potwierdzony smoke z LM Studio oraz zapisany minimalny kontrakt pod przyszle subagenty na lokalnym modelu.

# Zakres i kontekst
Dokumentacja ma odpowiadac na konkretne pytania:
- Czemu plik w `/models` nie wystarcza.
- Co wpisac w `config.toml` albo `custom_models.toml`.
- Skad Codex robi requesty dla LM Studio/Ollama.
- Jak sprawdzic, czy model dziala przez CLI.
- Jak przelaczyc sie z powrotem na premium.

Proponowany doc:
- `docs/custom-models.md`
- link z `docs/config.md` albo `README.md`, zgodnie z lokalnym stylem docs.

Subagent contract:
- Nie implementuj duzego subagent feature, jesli nie jest juz trywialny po `/model`.
- Zostaw w docs albo komentarzu architektonicznym jasny kontrakt przyszlego override:
  `spawn_agent(model_provider = "lmstudio", model = "google/gemma-4-26b-a4b")`.
- Jesli obecny spawn tool przyjmuje tylko `model`, zapisz follow-up: potrzebne pole provider/profile, bo samo `model` jest niejednoznaczne.

# Kroki
- [ ] Napisz docs dodawania modelu lokalnego do `/model`.
- [ ] Dodaj przyklad LM Studio z `google/gemma-4-26b-a4b`.
- [ ] Dodaj przyklad Ollama, jesli implementacja wspiera Ollama registry.
- [ ] Opisz endpointy: LM Studio `/api/v1/models`/`/v1/models`, Ollama `/api/tags`.
- [ ] Opisz smoke CLI dla `codex exec` i TUI smoke dla `/model`.
- [ ] Dodaj sekcje o ograniczeniach: Codex nie pobiera modeli i nie zarzadza `/models` filesystem.
- [ ] Zapisz kontrakt/follow-up dla subagentow na lokalnym providerze.
- [ ] Uruchom finalny gate i zapisz wyniki w `project.md`.

# Walidacja
Wymagane:

```bash
cd /home/sieciowiec/dev/rust/codex/codex-rs && just fmt
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo check -p codex-tui
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo check -p codex-core
```

Jesli LM Studio dziala:

```bash
curl -sS http://localhost:1234/api/v1/models
codex exec --skip-git-repo-check --oss --local-provider lmstudio -m google/gemma-4-26b-a4b "Return exactly: LOCAL_OK"
```

Docs acceptance:
- Nowy user rozumie, ze `/models` filesystem to storage LM Studio/Ollama, a `/model` to picker Codexa.
- Instrukcja ma gotowy blok TOML do wklejenia.
- Instrukcja mowi, jak wrocic na premium model.
