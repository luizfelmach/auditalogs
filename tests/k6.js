import { sleep } from 'k6';
import http from 'k6/http';

export const options = {
  duration: '1m',
  vus: 500,
};

function randomIP() {
  return `${Math.floor(Math.random() * 256)}.${Math.floor(Math.random() * 256)}.${Math.floor(Math.random() * 256)}.${Math.floor(Math.random() * 256)}`;
}

function elasticTimestamp() {
  return new Date().toISOString();
}

export default function () {
  const url = 'http://127.0.0.1:8080';

  const payload = JSON.stringify({
    ip: randomIP(),
    timestamp: elasticTimestamp(),
    mac: '00:00:00:00:00:00',
    port: Math.floor(Math.random() * 65536)
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  http.post(url, payload, params);
  sleep(0.01);
}

