// import { useEffect } from "react";
import "./App.css";
import SignUpPage from "./pages/SignUpPage";
import SignInPage from "./pages/SignInPage";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import LandingPage from "./pages/LandingPage";
import DashboardPage from "./pages/DashboardPage";

const App = () => {
  // useEffect(() => {
  //   fetch("/api/")
  //     .then((res) => res.text())
  //     .then((data) => console.log(data));
  // }, []);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/sign_up" element={<SignUpPage />} />
        <Route path="/sign_in" element={<SignInPage />} />
        <Route path="/dashboard" element={<DashboardPage />} />
      </Routes>
    </BrowserRouter>
  );
};

export default App;
