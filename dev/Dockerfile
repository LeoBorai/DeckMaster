FROM alpine:3.22

RUN apk add -f -u \
    bat \
    bash \
    ca-certificates \
    curl \
    fd-find \
    git \
    jq \
    npm \
    ripgrep \
    sudo \
    tree

RUN npm install -g @anthropic-ai/claude-code

RUN mkdir -p /workspaces/app

WORKDIR /workspaces/app
