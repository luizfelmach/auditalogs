import { sleep } from 'k6';
import http from 'k6/http';

let duration = "1m";
let vus = 1000;
let delay = 1;

 export const options = {
   duration,
   vus,
 };

export default function () {
  const url = 'http://127.0.0.1:8080';

  const payload = JSON.stringify({
    ip: "mock",
    mac: "mock",
    port: "mock"
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  http.post(url, payload, params);
  sleep(delay)
}
