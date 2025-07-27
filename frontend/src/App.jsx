import { useEffect } from "react";
import "./App.css";
import SignUpPage from "./pages/SignUp";

function App() {
  useEffect(() => {
    fetch("/api/")
      .then((res) => res.text())
      .then((data) => console.log(data));
  }, []);

  return (
    <>
      <SignUpPage />
    </>
  );
}

export default App;
