FROM node:latest AS build 

WORKDIR /app

COPY package*.json ./
RUN npm install
COPY . ./
RUN npm run build
RUN npm prune --omit=dev

EXPOSE 5000
ENV HOST=0.0.0.0
ENV PORT=5000
CMD ["node", "./build/index.js"]

