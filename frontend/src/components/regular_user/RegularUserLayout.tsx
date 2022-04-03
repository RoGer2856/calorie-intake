import { ReactElement, ReactNode } from "react";
import { IUserInfo } from "../../model/UserInfo";

export default function RegularUserLayout(props: {
    userInfo: IUserInfo,
    children: ReactNode
}): ReactElement {
    return (
        <>
            <h1>Hello {props.userInfo.username}!</h1>
            {props.children}
        </>
    );
}
