import React, { ReactElement, useCallback, useEffect, useState } from "react";
import useApi from "../../hooks/use-api";
import { IUserInfo } from "../../model/UserInfo";
import MyFoods from "../food/MyFoods";

export default function AllUserFoods(props: {
    myUserInfo: IUserInfo
}): ReactElement {
	const [, api] = useApi();

	const [userInfos, setUserInfos] = useState<IUserInfo[]>([]);
	const [selectedUserInfo, setSelectedUserInfo] = useState(props.myUserInfo);

	let fetchUserInfos = useCallback(async () => {
		const response = await api.getUserList();
		if (response !== null) {
			setUserInfos(response);
		}
	}, []);

	useEffect(() => {
		fetchUserInfos();
	}, [fetchUserInfos])

	async function selectHandler(event: React.FormEvent<HTMLSelectElement>) {
		const username = event.currentTarget.value;
		const userInfo = userInfos.find((userInfo: IUserInfo) => username === userInfo.username);
		if (userInfo !== undefined) {
			setSelectedUserInfo(userInfo);
		}
	}

	return (
		<>
			<form>
				<select
					onChange={selectHandler}
					className="form-select form-select-lg mb-3" aria-label=".form-select-lg example"
					defaultValue={props.myUserInfo.username}
				>
					<option value={props.myUserInfo.username}>{props.myUserInfo.username}</option>

					{userInfos
						.filter((userInfo: IUserInfo) => {
							return userInfo.username !== props.myUserInfo.username;
						})
						.map((userInfo: IUserInfo) => {
							return (
								<option
									key={userInfo.username}
									value={userInfo.username}
								>
									{userInfo.username}
								</option>
							);
						})}
				</select>
			</form>

			<MyFoods userInfo={selectedUserInfo!} />
		</>
	)
}