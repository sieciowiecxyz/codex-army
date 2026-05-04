# docs/plans

Ten katalog przechowuje artefakty dlugich, wieloetapowych projektow dla Codexa i innych agentow programistycznych.

Domyslny wzorzec dla projektu:

```text
<task-slug>/
  project.md
  milestones/
    01-<milestone>.md
    02-<milestone>.md
    03-<milestone>.md
```

`project.md` jest trwalym source of truth: cel, kontekst, zakres, walidacja, lista milestone'ow i log pracy.
Szczegoly wykonawcze trzymaj jako lekkie specy w `milestones/*.md`.
Po przejsciu walidacji milestone'u od razu przechodz do nastepnego nieukonczonego milestone'u, chyba ze projekt jest globalnie ukonczony albo zablokowany.
Internal Codex `update_plan` sluzy tylko do biezacego UI statusu i nie zastepuje `project.md`.
