import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";
import ReportScreen from "./ReportScreen";

export default function AdminApp(): ReactElement {
    return (
        <>
            <Routes>
                <Route path="/">
                    <Route index element={<ReportScreen />} />
                </Route>
            </Routes>
        </>
    )
}