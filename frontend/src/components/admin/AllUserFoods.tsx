import { ReactElement } from "react";
import useApi from "../../hooks/use-api";

export default function AllUserFoods(): ReactElement {
	const api = useApi();

	return (
		<>
			<p>AllUserFoods</p>
		</>
	)
}