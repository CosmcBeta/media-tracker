import { Route, Routes } from "react-router";
import Layout from "./components/Layout";
import ItemDetailPage from "./pages/ItemDetailPage";
import ItemsPage from "./pages/ItemsPage";
import ListDetailPage from "./pages/ListDetailPage";
import ListsPage from "./pages/ListsPage";
import SearchPage from "./pages/SearchPage";

function App() {
	return (
		<Routes>
			<Route element={<Layout />}>
				<Route path="/items" element={<ItemsPage />}></Route>
				<Route path="/items/:id" element={<ItemDetailPage />}></Route>
				<Route path="/lists" element={<ListsPage />}></Route>
				<Route path="/lists/:id" element={<ListDetailPage />}></Route>
				<Route path="/search" element={<SearchPage />}></Route>
			</Route>
		</Routes>
	);
}

export default App;
