import React, { ReactElement, useCallback, useEffect, useState } from "react";
import { createSearchParams, useSearchParams } from "react-router-dom";
import useApi from "../../hooks/use-api";
import { IUserInfo } from "../../model/UserInfo";
import MyFoods from "../food/MyFoods";
import UseApiView from "../UseApiView";

export default function AllUserFoods(props: {
    myUserInfo: IUserInfo
}): ReactElement {
	const api = useApi();
	const [userInfos, setUserInfos] = useState<IUserInfo[]>([]);
	let [searchParams, setSearchParams] = useSearchParams();
	const [selectedUserInfo, setSelectedUserInfo] = useState<IUserInfo | null>(null);

	function setSelectedUserByUsername(username: string) {
		let newSearchParams = createSearchParams(searchParams);
		newSearchParams.set("username", username);
		setSearchParams(newSearchParams);

		if (userInfos.length !== 0) {
			const userInfo = userInfos.find((userInfo: IUserInfo) => username === userInfo.username);
			if (userInfo !== undefined && userInfo !== selectedUserInfo) {
				setSelectedUserInfo(userInfo);
			}
		}
	}

	useEffect(() => {
		let username = searchParams.get("username");
		if (username === null) {
			setSelectedUserByUsername(props.myUserInfo.username);
		} else {
			setSelectedUserByUsername(username);
		}
	}, [searchParams, userInfos, props.myUserInfo.username, setSelectedUserInfo]);

	function changeHandler(event: React.FormEvent<HTMLSelectElement>) {
		const username = event.currentTarget.value;
		setSelectedUserByUsername(username);
	}

	let fetchUserInfos = useCallback(async () => {
		const response = await api.getUserList();
		if (response !== null) {
			setUserInfos(response);
		}
	}, []);

	useEffect(() => {
		fetchUserInfos();
	}, [fetchUserInfos])

	return (
		<>
			<UseApiView api={api} >
				<form>
					<select
						onChange={changeHandler}
						className="form-select form-select-lg mb-3" aria-label=".form-select-lg example"
						value={selectedUserInfo?.username}
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

				{selectedUserInfo !== null
					?
					<MyFoods userInfo={selectedUserInfo!} />
					:
					<></>
				}
			</UseApiView>
		</>
	)
}