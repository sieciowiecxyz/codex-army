# Cel milestone'u - registry custom OSS models
Codex ma stabilny, testowany sposob wczytywania user-defined lokalnych modeli OSS, ktore pozniej mozna pokazac w `/model`.

# Zakres i kontekst
Preferowany kontrakt configu:

```toml
[[custom_models]]
name = "Gemma 4 26B A4B"
model = "google/gemma-4-26b-a4b"
provider = "lmstudio"
base_url = "http://localhost:1234/v1"
source = "manual"
```

Minimalne wymagane pola:
- `model`: identyfikator przekazywany do providera, np. `google/gemma-4-26b-a4b`.
- `provider`: `lmstudio` albo `ollama`.

Opcjonalne pola:
- `name`: label w UI, fallback do `model`.
- `base_url`: override endpointu providera, fallback do wbudowanych portow.
- `source`: `manual`, `lmstudio-discovered`, `ollama-discovered`; w pierwszym kroku moze zostac enum/string tylko do UI.
- `description`: opcjonalny tekst pomocniczy w pickerze.

Fallback architektoniczny:
- Jesli zmiana `ConfigToml` grozi duzym konfliktem z upstream, uzyj `CODEX_HOME/custom_models.toml`:

```toml
[[models]]
name = "Gemma 4 26B A4B"
model = "google/gemma-4-26b-a4b"
provider = "lmstudio"
base_url = "http://localhost:1234/v1"
```

Nie implementuj obu wariantow bez potrzeby. Wybierz jeden i opisz decyzje w `project.md`.

# Kroki
- [ ] Znajdz definicje `ConfigToml`, `ConfigProfile`, `oss_provider` i parsery configu.
- [ ] Sprawdz, czy `model_provider = "lmstudio"` jest traktowane jak provider runtime, czy potrzebny jest tylko `oss_provider`.
- [ ] Zaprojektuj maly typ `CustomModelConfig`/`CustomModelEntry` w najwlasciwszej crate, unikaj dokladania logiki do `codex-core`, jesli mozna ja trzymac w config/model-provider-info.
- [ ] Dodaj parsing recznych wpisow z configu albo micro pliku.
- [ ] Dodaj walidacje providerow: akceptuj tylko `lmstudio` i `ollama` na start.
- [ ] Dodaj unit tests na parse, defaulty, brak configu, bledny provider i minimalny wpis.
- [ ] Jesli ruszony `ConfigToml`, uruchom `just write-config-schema` i sprawdz diff schema.
- [ ] Zaktualizuj `project.md` decyzja: `config.toml` vs `custom_models.toml`.

# Walidacja
Wymagane po zmianach:

```bash
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo test -p codex-core config
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo check -p codex-core
```

Jesli zmieniono schema configu:

```bash
cd /home/sieciowiec/dev/rust/codex/codex-rs && just write-config-schema
```

Reality check:
- Pusty config nie zmienia zachowania Codexa.
- Wpis `provider = "lmstudio"` i `model = "google/gemma-4-26b-a4b"` jest dostepny jako structured entry dla TUI.
- Bledny provider daje czytelny blad albo jest ignorowany z ostrzezeniem, zgodnie z lokalnym stylem configu.
