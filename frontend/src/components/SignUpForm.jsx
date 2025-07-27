import { useState } from "react";
import { Box, Typography, TextField, Button } from "@mui/material";

const SignUpForm = () => {
  const [form, setForm] = useState({
    name: "",
    email: "",
    password: "",
    confirmPassword: "",
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

    const newErrors = {};

    if (form.name.trim().length < 3) {
      newErrors.name = "Player name must be at least 3 characters.";
    }

    if (!form.email.includes("@") || !form.email.includes(".")) {
      newErrors.email = "Please enter a valid email.";
    }

    if (form.password.length < 8) {
      newErrors.password = "Password must be at least 8 characters.";
    }

    if (form.password != form.confirmPassword) {
      newErrors.confirmPassword = "Passwords must match.";
    }

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors);
      return;
    }

    setErrors({});
    console.log(form);

    // TODO: send to backend
    fetch("/api/sign_up", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        name: form.name,
        email: form.email,
        password: form.password,
      }),
    })
      .then(async (res) => {
        const data = await res.json();
        return [res.ok, data];
      })
      .then(([ok, data]) => {
        if (ok) {
          console.log("Success:", data);
        } else {
          console.log("Error:", data);
        }
        // TODO: redirect
      })
      .catch((err) => {
        console.error("Request failed", err);
      });
  };

  return (
    <Box component="form" onSubmit={handleSubmit}>
      <Typography variant="h4">Sign Up</Typography>
      <TextField
        fullWidth
        label="Player Name"
        name="name"
        value={form.name}
        onChange={handleChange}
        error={!!errors.name}
        helperText={errors.name}
      />
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
      <TextField
        fullWidth
        label="Confirm Password"
        type="password"
        name="confirmPassword"
        value={form.confirmPassword}
        onChange={handleChange}
        error={!!errors.confirmPassword}
        helperText={errors.confirmPassword}
      />
      <Button variant="outlined" type="submit">
        Submit
      </Button>
    </Box>
  );
};

export default SignUpForm;
