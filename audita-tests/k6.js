import { sleep } from 'k6';
import http from 'k6/http';

 export const options = {
   duration: "1m",
   vus: 20
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
  // sleep(0.1)
}
