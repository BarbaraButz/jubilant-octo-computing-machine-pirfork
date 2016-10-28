Übungsaufgaben zu „Programmieren in Rust“
=========================================

**Gruppe**:

- *Susi Sorglos*
- *Willi Wacker*

---

In diesem Repository werden die Lösungen der oben genannten Gruppe zu den
Programmieraufgaben in der Vorlesung „Programmieren in Rust“ gesammlt. Dazu
muss mindestens ein Gruppenmitglied diese Repository forken. Von dem Fork
wird später dann pro Aufgabenblatt ein *Pull Request* (PR) geöffnet.

Neue Übungsaufgaben werden in [diesem Repository][1] veröffentlicht. Es kann
vorkommen, dass mit der Aufgabenstellung schon ein paar Quelldateien vorgegeben
werden; diese müsst ihr dann von dem genannten Repository in euern Fork
kopieren.

Die Lösungen müssen in einer bestimmten Ordnerstruktur liegen, die wie folgt
aussieht:

```
README.md
sheet1/
    task1/
    task2/
sheet2/
    task1/
    task2/
```

In den `taskN`-Unterordnern muss dann eure Lösung für die entsprechende Aufgabe
liegen.

### Travis-CI

Um das Arbeiten zu erleichtern, ist für alle Lösungsrepositories ein Continuous
Integration Service aufgesetzt worden. Jedes mal, wenn ein PR geöffnet
oder aktualisiert wird, laufen eine Reihe von Tests durch, die den Codestil
prüfen, alle Rust Dateien kompilieren und alle Unit-Tests ausführen. Manuell
könnt ihr all diese Tests durchlaufen lassen, indem ihr im Wurzelverzeichnes
des Repositories folgenden Befehl ausführt (nur Linux und Verwandte!):

```
./ci/run-all.sh
```

Jeder PR hat also einen Status: *passed*, *failed* oder *pending*. Euer PR zum
Einreichen der Aufgaben muss bis Sonntag Abend den Status *passed* erreicht
haben, also plant genug Zeit zum Verbessern von kleinen Fehlern ein und öffnet
den PR nicht erst kurz vor Schluss.


[1]: https://github.com/LukasKalbertodt/programmieren-in-rust
