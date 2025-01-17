+++
title = "Thread-Safe Memory Allocator "

[extra]
image = "https://upload.wikimedia.org/wikipedia/commons/e/e7/Zuordnung_von_Inhalten_eines_C-Programms_zu_Speicherbereichen_%28Segmenten%29.png"
link = "https://github.com/DavidYang829/DukeECE568-HW1"
technologies = ["C", "Thread-safe"]
+++

Implemented Malloc Library, experimented with First Fit and Best Fit allocation strategies and evaluated trade
offs.

Made it thread-safe, with a locked version (pthread mutex) and a lock-free version (Thread Local Storage).

Optimized it with Best-Fit strategy which reduced memory fragmentation rate from 15 % to 2 %.

Decreased execution time of Thread Local Storage from 1.8s to 0.2s.