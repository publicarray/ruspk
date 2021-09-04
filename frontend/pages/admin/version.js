import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatBoolean } from '../../utils';

export async function getServerSideProps({ query }) {
    const url = `http://127.0.0.1:8080/api/version`
    const page = query.page || 1; //if page empty we request the first page
    const res = await fetch(`${url}?page=${page}&size=15`)
    const data = await res.json()

    if (!data) {
        return {
            notFound: true,
        }
    }

    return {
        props: {
            data,
            currentPage: page,
            pageCount: 100
        }
    }
}

export default function VersionPage({data, currentPage, pageCount}) {
    let [isOpen, setIsOpen] = useState(true);

    function closeModal() {
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    // const data = [
    //     { id: 1, package: "python38", version: "3.8.11", revision: "4", beta: "false", services: "", insert_date: "2021-08-31 22:42:37.109035", all_builds_active: "true", install_wizard: "false", upgrade_wizard: "false", startable: "false"  },
    // ];

    const columns = [
        { Header: 'ID', accessor: 'id' },
        { Header: 'Package', accessor: 'package' },
        { Header: 'Upstream Version', accessor: 'upstream_version' },
        { Header: 'Revision', accessor: 'revision' },
        { Header: 'Beta', accessor: 'beta' },
        // { Header: 'Services', accessor: 'services' },
        { Header: 'Insert Date', accessor: 'insert_date' },
        { Header: 'All Builds Active', accessor: row => formatBoolean(row.all_builds_active) },
        { Header: 'Install Wizard', accessor: row => formatBoolean(row.install_wizard) },
        { Header: 'Upgrade Wizard', accessor: row => formatBoolean(row.upgrade_wizard) },
        { Header: 'Startable', accessor: row => formatBoolean(row.startable) },
    ];

    return (
        <Layout>
            <h1>Version</h1>
            <Table columns={columns} data={data} currentPage={currentPage} pageCount={pageCount}></Table>
            <Button>Add Version</Button>
        </Layout>
    );
}
