import { useState } from "react";
import { Box, Typography, TextField, Button } from "@mui/material";
import Header from "./Header";
import { useNavigate } from "react-router-dom";

const SignInForm = () => {
  const navigate = useNavigate();

  const [form, setForm] = useState({
    email: "",
    password: "",
  });

  const [errors, setErrors] = useState({});

  const handleChange = (e) => {
    setForm((prev) => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();

    fetch("/api/sign_in", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        email: form.email.trim(),
        password: form.password,
      }),
    })
      .then(async (res) => {
        const data = await res.json();

        if (res.ok) {
          navigate(data.redirect_to); // TODO: maybe change backend at some point to eliminate redirect responses
          return;
        } else {
          console.log("Error:", data);
        }
      })
      .catch((err) => {
        console.error("Request failed", err);
      });
  };

  return (
    <Box>
      <Header />
      <Box component="form" role="form" onSubmit={handleSubmit}>
        <Typography variant="h4" sx={{ padding: "20px" }}>
          Sign In
        </Typography>
        <TextField
          fullWidth
          label="Email"
          name="email"
          value={form.email}
          onChange={handleChange}
          error={!!errors.email}
          helperText={errors.email}
        />
        <TextField
          fullWidth
          label="Password"
          type="password"
          name="password"
          value={form.password}
          onChange={handleChange}
          error={!!errors.password}
          helperText={errors.password}
        />
        <Button variant="outlined" type="submit">
          Submit
        </Button>
      </Box>
    </Box>
  );
};

export default SignInForm;
