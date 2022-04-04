import { ReactElement } from "react";
import { Navigate, Route, Routes } from "react-router-dom";
import { IUserInfo } from "../../model/UserInfo";
import MyFoods from "../food/MyFoods";
import PageNotFound from "../PageNotFound";
import RegularUserLayout from "./RegularUserLayout";

export default function RegularUserApp(props: {
    userInfo: IUserInfo
}): ReactElement {
    return (
        <>
            <RegularUserLayout userInfo={props.userInfo}>
                <Routes>
                    <Route path="/">
                        <Route index element={<Navigate replace to="/my-consumption" />} />
                        <Route path="/my-consumption" element={<MyFoods maxCaloriesPerDay={props.userInfo.maxCaloriesPerDay} />} />
                    </Route>
                    <Route path="*" element={<PageNotFound />} />
                </Routes>
            </RegularUserLayout>
        </>
    )
}