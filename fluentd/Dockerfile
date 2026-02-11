FROM fluent/fluentd:edge-debian

USER root

# PostgreSQL client libs for fluent-plugin-postgres
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev \
  && rm -rf /var/lib/apt/lists/*

# Install Fluentd PostgreSQL output plugin
RUN fluent-gem install fluent-plugin-postgres

COPY fluent.conf /fluentd/etc/fluent.conf

USER fluent
