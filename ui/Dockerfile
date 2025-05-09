FROM node:20-alpine

WORKDIR /app

RUN npm install -g pnpm

COPY package*.json ./
COPY pnpm-lock.yaml ./

RUN pnpm install --no-frozen-lockfile

COPY . .

RUN npm install -g serve

EXPOSE 8081

CMD ["sh", "-c", "pnpm build && serve -s dist -l 8081"]
