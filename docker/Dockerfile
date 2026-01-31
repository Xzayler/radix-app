FROM node:22-alpine

WORKDIR /app

# pnpm
RUN corepack enable

COPY /web/package.json ../web/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY /web .

RUN pnpm build

CMD ["pnpm", "start"]