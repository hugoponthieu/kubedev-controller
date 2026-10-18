FROM alpine/git:latest

USER root

COPY pull-project.sh /usr/local/bin/pull-project

RUN chmod 0555 /usr/local/bin/pull-project

ENTRYPOINT ["/usr/local/bin/pull-project"]
