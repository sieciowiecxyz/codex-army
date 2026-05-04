# Cel milestone'u - Local/OSS w `/model`
`/model` pokazuje lokalne modele OSS obok modeli premium i pozwala przelaczyc aktywna rozmowe przez atomowy wybor `provider + model`.

# Zakres i kontekst
Znane miejsca startowe:
- `codex-rs/tui/src/chatwidget.rs` zawiera aktualny `/model` picker i funkcje w okolicy `model_selection_actions`.
- `codex-rs/tui/src/oss_selection.rs` juz obsluguje wybor OSS providera przy starcie.
- `codex-rs/tui/src/lib.rs` i `codex-rs/exec/src/lib.rs` juz rozwiazuja `oss_provider` dla startu sesji.
- `codex-rs/model-provider-info/src/lib.rs` definiuje `lmstudio` i `ollama`.

Preferowany kierunek:
- Dodaj nowy maly modul TUI, np. `codex-rs/tui/src/model_picker/custom_models.rs` albo podobny, ktory buduje Local/OSS entries.
- W `chatwidget.rs` zostaw tylko cienkie podlaczenie do istniejacego pickera.
- Nie mieszaj dynamicznego network fetch z renderem synchronicznym, jesli moze zawiesic UI. Reczny registry wystarczy do pierwszego DONE.
- Jesli dodajesz dynamiczne discovery:
  - LM Studio: `GET http://localhost:1234/api/v1/models`, fallback `GET http://localhost:1234/v1/models`.
  - Ollama: `GET http://localhost:11434/api/tags`.
  - Timeout krotki, np. 300-800 ms, wynik cache'owany albo traktowany jako best-effort.

Wazna semantyka:
- Dla premium wyboru: provider wraca na OpenAI.
- Dla local wyboru: provider staje sie `lmstudio` albo `ollama`; model staje sie wybranym `model`.
- Nie zapisuj automatycznie wyboru do globalnego configu, chyba ze istniejacy `/model` juz tak robi. Trzymaj sie obecnego zachowania pickera.

# Kroki
- [ ] Przeczytaj aktualne flow `/model`: akcje, eventy, update configu aktywnej sesji, testy/snapshoty.
- [ ] Zidentyfikuj minimalny typ reprezentujacy wybor: `Premium { model, effort }` vs `LocalOss { provider, model, label, base_url }`.
- [ ] Dodaj builder Local/OSS entries z registry z milestone 2.
- [ ] Rozszerz picker o sekcje/group header `Local / OSS` bez psucia aktualnego ekranu `Select Model and Effort`.
- [ ] Zaimplementuj handler wyboru lokalnego modelu tak, aby zmienial provider i model razem.
- [ ] Zaimplementuj powrot na premium provider przy wyborze modelu OpenAI.
- [ ] Dodaj testy/snapshoty TUI dla: brak custom modeli, jeden model LM Studio, model aktywny lokalnie, powrot na premium.
- [ ] Zaktualizuj `project.md` o dokladne miejsca kodu i walidacje.

# Walidacja
Wymagane:

```bash
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo check -p codex-tui
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo test -p codex-tui model
```

Jesli zmieniaja sie snapshoty:

```bash
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo insta pending-snapshots -p codex-tui
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo insta accept -p codex-tui
```

Manual smoke:
- Dodaj Gemma local do configu testowego.
- Uruchom TUI.
- Otworz `/model`.
- Potwierdz, ze `Local / OSS` zawiera Gemma.
- Wybierz Gemma i sprawdz, ze aktywna sesja raportuje provider `lmstudio`.
- Wybierz `gpt-5.5` i sprawdz, ze provider wraca na OpenAI.
