import { Box, Button, Typography } from "@mui/material";
import Header from "../components/Header";
import { Link } from "react-router-dom";

const LandingPage = () => {
  const TextBox = ({ title, description, link }) => {
    return (
      <Box
        component={Link}
        to={link}
        sx={{
          textAlign: "left",
          border: "1px black solid",
          padding: "10px",
          width: "100%",
          display: "flex",
          justifyContent: "space-between",
        }}
      >
        <Box sx={{ width: "100vh" }}>
          <Typography>{title}</Typography>
          <Typography>{description}</Typography>
        </Box>
        <Box>
          <Button>&gt;</Button>
        </Box>
      </Box>
    );
  };

  return (
    <>
      <Header />
      <Typography variant="h3" sx={{ padding: "20px" }}>
        Landing Page.
      </Typography>
      <TextBox
        title="REGISTER"
        description="Don't have an account? Create one now!"
        link="sign_up"
      />
      <TextBox
        title="LOGIN"
        description="Sign into your account"
        link="sign_in"
      />
    </>
  );
};

export default LandingPage;
