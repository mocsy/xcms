# Node setup
   * Start with [Ubuntu server inital setup](https://www.digitalocean.com/community/tutorials/initial-server-setup-with-ubuntu-18-04)
   * Install a reverse proxy [nginx](https://www.digitalocean.com/community/tutorials/how-to-install-nginx-on-ubuntu-18-04)
   * Set up nginx config based on example in config/nginx_80
   * To use Let's encrypt SSL certs, install [certbot](https://certbot.eff.org/lets-encrypt/ubuntubionic-nginx) and then call `sudo certbot --nginx` for easy https setup
   * Fix cerbot generated nginx config like config/nginx_certbot
