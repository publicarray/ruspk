import Layout from "../../components/layout-admin";
import Button from "../../components/button";
import DeleteBtn from "../../components/delete-btn";
import TablePaginate from "../../components/table-paginate";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatBoolean, API, API_VER } from '../../utils';
import { postJsonForm } from "../../utils";
import { useRouter } from 'next/router'
import { createColumnHelper } from "@tanstack/react-table";

export const runtime = 'edge';

export default function VersionPage() {
    const url = `${API}/${API_VER}/version`
    const router = useRouter()
    const [data, setData] = useState([]);

    let del = async function (id, index) {
        const response = await fetch(`${url}/${id}`, {
            headers: {
                'Authorization': 'Bearer ' + localStorage.getItem("jwt")
            },
            method: "DELETE",
        });
        if (response.ok) {
            data.splice(index, 1) // update table
            router.push("/admin/version", undefined, {shallow: true}) // force refresh of internal data
        }
    }

    const columnHelper = createColumnHelper();
    const columns = [
        columnHelper.accessor("id"),
        columnHelper.accessor("package"),
        columnHelper.accessor("upstream_version", {
            header: "Upstream Version"
        }),
        columnHelper.accessor("revision"),
        columnHelper.accessor("report_url", {
            header: "Beta",
            cell: (info) => info.getValue() ? "Yes" : "No"
        }),
        columnHelper.accessor("insert_date", {
            header: "Insert Date",
        }),
        columnHelper.accessor("all_builds_active", {
            header: "All Builds Active",
            cell: (info) => formatBoolean(info.getValue())
        }),
        columnHelper.accessor("install_wizard", {
            header: "Install Wizard",
            cell: (info) => formatBoolean(info.getValue())
        }),
        columnHelper.accessor("upgrade_wizard", {
            header: "Upgrade Wizard",
            cell: (info) => formatBoolean(info.getValue())
        }),
        columnHelper.accessor("startable", {
            cell: (info) => formatBoolean(info.getValue())
        }),
        columnHelper.accessor("actions", {
            header: "Actions",
            cell:  (info) => {
            return (
                <div>
                    {/* <span onClick={() => edit(row)}>
                        <i className="far fa-edit action mr-2">Edit</i>
                    </span> */}
                    <span onClick={i => del(info.row.original.id, info.row.index)}>
                        <DeleteBtn></DeleteBtn>
                    </span>
                </div>
            );
            },
        }),
    ];

    return (
        <Layout>
            <h1>Version</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
        </Layout>
    );
}
