# Cel milestone'u - sync z upstream i baseline walidacji
Repo jest zsynchronizowane przed implementacja, lokalne QoL commity sa zachowane, a stan kompilacji przed zmiana jest znany i zapisany w `project.md`.

# Zakres i kontekst
Pracuj w `/home/sieciowiec/dev/rust/codex`.

Ten milestone nie implementuje feature'a. Ma ograniczyc ryzyko przyszlych konfliktow i uniknac sytuacji, w ktorej feature maskuje juz istniejacy blad kompilacji.

Aktualny stan rozpoznany przed planem: `main...origin/main [ahead 11]`.

# Kroki
- [ ] Uruchom `git status --short --branch` i zapisz stan w logu projektu.
- [ ] Sprawdz remote'y przez `git remote -v`.
- [ ] Uruchom `git fetch --all --prune`.
- [ ] Ustal, czy synchronizowac z `origin/main`, `upstream/main`, czy innym remote; preferuj remote zgodny z lokalnym forkiem.
- [ ] Wykonaj merge albo rebase bez gubienia lokalnych QoL commitow. Jesli sa konflikty, rozwiaz je minimalnie i zapisz decyzje w logu.
- [ ] Uruchom baseline `cargo check -p codex-tui` w `codex-rs`.
- [ ] Uruchom baseline `cargo check -p codex-core` w `codex-rs`.
- [ ] Zapisz wynik sync i baseline w `project.md`.

# Walidacja
Wymagane:

```bash
git -C /home/sieciowiec/dev/rust/codex status --short --branch
git -C /home/sieciowiec/dev/rust/codex log --oneline --decorate -n 12
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo check -p codex-tui
cd /home/sieciowiec/dev/rust/codex/codex-rs && cargo check -p codex-core
```

Milestone jest zaliczony tylko wtedy, gdy log projektu mowi jasno:
- do czego zsynchronizowano repo,
- czy lokalne commity zostaly zachowane,
- czy baseline kompiluje sie przed implementacja.
