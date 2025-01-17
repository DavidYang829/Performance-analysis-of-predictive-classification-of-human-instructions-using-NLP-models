+++
title = "HTTP Caching Proxy Server"

[extra]
image = "https://upload.wikimedia.org/wikipedia/commons/c/c4/Proxy2.jpg"
link = "https://github.com/DavidYang829/DukeECE568-HW2"
technologies = ["C++", "Network", "TCP Socket", "Multi-Thread"]
+++

Established deployed an HTTP caching proxy server to handle 5000+ requests per second including GET, POST, and CONNECT methods using Socket Programming and Docker within a 2-person team.

Implemented RAII technique to enhance resource management and provide classes with a strong exception safety
guarantee using C++ 11.

Enhanced server request processing efficiency by 40% through the implementation of a multi-threading approach
and Mutex lock strategy.

Improved performance by implementing Least Recently Used (LRU) caching mechanism by storing frequently
accessed URL.and HTTP responses with expire-checking and re-validation adhering to RFC7234.