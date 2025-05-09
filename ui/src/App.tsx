import { Header } from "./components/header";
import { SearchInterface } from "./components/search-interface";

function App() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-50 to-white">
      <Header />
      <SearchInterface />
    </div>
  );
}

export default App;
