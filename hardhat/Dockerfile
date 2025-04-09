FROM node:20

WORKDIR /app

COPY package.json .
COPY hardhat.config.ts .
COPY contracts ./contracts
COPY ignition ./ignition
COPY tsconfig.json .

RUN npm install

EXPOSE 8545

CMD ["sh", "-c", "npx hardhat node & sleep 5 && npx hardhat compile && npx hardhat ignition deploy ./ignition/modules/Auditability.ts --network localhost && sleep infinity"]
