nrsysmond-config --set license_key=NEW_RELIC_LICENSE_KEY
/etc/init.d/newrelic-sysmond start

diesel migration run

ROCKET_PORT=$PORT fake-news-api
