FROM alpine:3.24

RUN apk add --no-cache \
      bash \
      cargo \
      git \
      py3-pip \
      rustfmt \
      shfmt \
      taplo \
  && pip install --no-cache-dir --break-system-packages mdformat \
  && git config --system --add safe.directory '*'

WORKDIR /repo
