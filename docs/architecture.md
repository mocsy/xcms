# Requirements
## Backend design goals
(not necessarily met at this point)

* Minimal overhead:
    * no copy: move or ref
    * compiled: prefer compile time constructs wherever makes sense
* DB access is done only in backend
* Requires authorization for everything
* Must run both standalone and behind a webserver like nginx
* Does not serve files other than ./static
    * only serves data with REST
    * it must not be possible to access temporary files, cache files, or config files used by the backend
    

## Behind a CDN
* Generates static html pages from DB by querying the Backend
