import { Route, Routes } from "react-router";
import Layout from "./components/Layout";
import HomePage from "./pages/HomePage";
import ItemsPage from "./pages/ItemsPage";
import ListDetailPage from "./pages/ListDetailPage";
import ListsPage from "./pages/ListsPage";
import SearchPage from "./pages/SearchPage";
import SettingsPage from "./pages/SettingsPage";

function App() {
	return (
		<Routes>
			<Route element={<Layout />}>
				<Route path="/" element={<HomePage />}></Route>
				<Route path="/settings" element={<SettingsPage />}></Route>
				<Route path="/items" element={<ItemsPage />}></Route>
				<Route path="/lists" element={<ListsPage />}></Route>
				<Route path="/lists/:id" element={<ListDetailPage />}></Route>
				<Route path="/search" element={<SearchPage />}></Route>
			</Route>
		</Routes>
	);
}

export default App;
