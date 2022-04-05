import { ReactElement } from "react";

export default function ErrorView(props: {
	errorMessage: string | null,
}): ReactElement {
	if (props.errorMessage === null) {
		return (
			<></>
		);
	} else {
		return (
			<>
				<p className="alert alert-danger">
					{props.errorMessage}
				</p>
			</>
		);
	}
}