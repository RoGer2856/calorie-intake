import { ReactElement, ReactNode } from "react";
import { UseApiHandler } from "../hooks/use-api";
import ErrorView from "./ErrorView";
import Loading from "./Loading";

export default function UseApiView(props: {
	api: UseApiHandler,
	children: ReactNode,
}): ReactElement {
	return (
		<>
			{props.api.isLoading
				?
				<Loading />
				:
				<>
					{props.api.errorMessage === null
						?
						props.children
						:
						<ErrorView errorMessage={props.api.errorMessage} />
					}
				</>
			}
		</>
	);
}