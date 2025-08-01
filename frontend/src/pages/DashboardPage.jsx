import { Box, Typography } from "@mui/material";
import Header from "../components/Header";

const DashboardPage = () => {
  return (
    <Box>
      <Header />
      <Typography variant="h3" sx={{ padding: "20px" }}>
        Dashboard
      </Typography>
      Hello World
    </Box>
  );
};

export default DashboardPage;
