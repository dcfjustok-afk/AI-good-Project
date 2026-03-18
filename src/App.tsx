import { Route, Routes } from "react-router-dom";
import { AppLayout } from "./components/app-layout";
import { FavoritesPage } from "./pages/favorites-page";
import { HomePage } from "./pages/home-page";
import { NotFoundPage } from "./pages/not-found-page";
import { ProjectDetailPage } from "./pages/project-detail-page";

function App() {
  return (
    <Routes>
      <Route element={<AppLayout />}>
        <Route index element={<HomePage />} />
        <Route path="projects/:owner/:repo" element={<ProjectDetailPage />} />
        <Route path="favorites" element={<FavoritesPage />} />
      </Route>
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}

export default App;
