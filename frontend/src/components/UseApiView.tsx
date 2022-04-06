import { ReactElement, ReactNode } from "react";
import { UseApiFeedback } from "../hooks/use-api";
import ErrorView from "./ErrorView";
import Loading from "./Loading";

export default function UseApiView(props: {
	apiFeedback: UseApiFeedback,
	children: ReactNode,
}): ReactElement {
	return (
		<>
			{props.apiFeedback.isLoading
				?
				<Loading />
				:
				<>
					{props.apiFeedback.errorMessage === null
						?
						props.children
						:
						<ErrorView errorMessage={props.apiFeedback.errorMessage} />
					}
				</>
			}
		</>
	);
}