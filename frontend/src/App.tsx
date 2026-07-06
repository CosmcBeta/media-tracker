import { Route, Routes } from "react-router";
import Layout from "./components/Layout";
import ListDetailPage from "./pages/ListDetailPage";
import ListsPage from "./pages/ListsPage";

function App() {
	return (
		<Routes>
			<Route element={<Layout />}>
				<Route path="/" element={<ListsPage />}></Route>
				<Route path="/lists" element={<ListsPage />}></Route>
				<Route path="/lists/:id" element={<ListDetailPage />}></Route>
			</Route>
		</Routes>
	);
}

export default App;
