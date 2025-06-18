import { useState } from "react";
import { LoginPage } from "./pages/login";
import { Dashboard } from "./pages/dashboard";

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  if (!isAuthenticated) {
    return <LoginPage onLogin={() => setIsAuthenticated(true)} />;
  }

  return <Dashboard />;
}

export default App;
