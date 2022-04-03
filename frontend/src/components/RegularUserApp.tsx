import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";
import MyFoods from "./MyFoods";

export default function RegularUserApp(): ReactElement {
    return (
        <>
            <Routes>
            <Route path="/">
                <Route index element={<MyFoods />} />
            </Route>
            </Routes>
        </>
    )
}