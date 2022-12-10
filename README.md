# Scrolling shooter

## Autorzy
- Mateusz Malinowski (gr 4)

## Opis
Od dwóch lat chciałem napisać scrolling shootera i wygląda na to, że w końcu go napiszę.

Gra tego gatunku, w którą dużo grałem w dzieciństwie to
[Chicken Invaders 4](https://www.youtube.com/watch?v=BjIFXQgty3k), więc zapewne będę się nią inspirował.

Przeciwnicy będą pojawiać się falami z bossem co 10 fal. Fale będą generowane losowo. Liczba fal będzie
nieskończona. Gra będzie się toczyć do utraty wszystkich żyć przez gracza.

## Funkcjonalność
- Latanie statkiem kosmicznym
- Strzelanie
- Możliwość zapisywania i wczytywania stanu gry
  - autosave co falę
  - ręczy zapis zapisuje stan po przejściu ostatniej fali
- Punktacja
- Ulepszanie broni
- Różne rodzaje broni

## Propozycja podziału na części
### Pierwsza część:
- Poruszanie się ✅
- Strzelanie ✅
- Niszczenie przeciwników ✅
- Punktacja ✅
- Możliwość zapisywania i wczytywania stanu gry ✅

### Druga część:
- Ulepszanie broni ✅
- Różne rodzaje broni ✅
- Ładne animacje ✅
  - niszczenie przeciwników ✅
  - przewijające się tło ✅
- Dźwięki ❌
- Poprawienie organizacji kodu ✅
- Poprawienie poruszanie się przeciwników ❌
- Poprawienie saveów ❌
  - zapisywanie w trakcie fali ❌
  - możliwość wyboru savea ❌
  - quicksave ❌
- Balans rozgrywki ✅/❌

## Podsumowanie pierwszej części
Tworząc grę, korzystałem z tego [tutorialu](https://www.youtube.com/watch?v=j7qHwb7geIM),
[cheat booka](https://bevy-cheatbook.github.io/introduction.html) oraz przykładów
[ui](https://bevyengine.org/examples/ui/ui/) i
[state](https://github.com/bevyengine/bevy/blob/v0.7.0/examples/ecs/state.rs). Udało mi się zrealizować wszystkie
zaplanowane na tę część zadania. Ogólnie rzecz biorąc, gra jest grywalna. Bardzo przyjemnie tworzy się gry w bevy.

### Sterowanie
- strzałki — poruszanie się
- spacja — strzelanie lub odpazuowanie
- escape lub p — pauza

### Zapisywanie gry
- można zapisać grę, ale ścieżka pliku jest zahadrkodowana, więc można mieć tylko jeden zapis (kolejny zapis nadpisuje
poprzedni)
- zapisywany jest numer fali, liczba żyć oraz liczba punktów w formacie tekstowym

### Inne
- przeciwnicy pojawiają się losowo, mają losową prędkość, poruszają się po trajektorii rombu
- przeciwnicy strzelają losowo, średnio powinien być jeden strzał na sekundę na ekranie 
- gracz może strzelać tak szybko, jak jest w stanie naciskać spację
- gracz ma 3 życia, ale nie jest to nigdzie wyświetlane

## Podsumowanie drugiej części
Została dodana nowa broń — shotgun. Bronie można ulepszać, każde ulepszenie zwiększa liczbę wystrzeliwanych
pocisków o 1. Została dodana możliwość zbierania ulepszeń. Zostały dodane animacje niszczenia przeciwników
i przewijanie tła. Pozostałe zaplanowane rzeczy nie zostały zrealizowane, ponieważ postanowiłem skupić się
na nauce do sesji.

### Ulepszenia
- ![hp](/assets/hp.png) - zwiększa ilość żyć o 1
- ![power_up](/assets/power_up.png) - zwiększa poziom broni o 1
- ![blue_weapon](/assets/blue_weapon.png) - zmienia broń na lasery
- ![green_weapon](/assets/green_weapon.png) - zmienia broń na shotgun

## Biblioteki
- Bevy
- rand
