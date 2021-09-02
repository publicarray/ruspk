import { useTable } from 'react-table'
import DetailBtn from './detail-btn'
import EditBtn from './edit-btn'
import DeleteBtn from './delete-btn'
// https://react-table.tanstack.com/docs/quick-start
// const data = [
//     { id: 1, arch: "noarch" },
//     { id: 2, arch: "ppc824x" },
//     { id: 3, arch: "ppc854x" },
//     { id: 4, arch: "ppc853x" },
//     { id: 5, arch: "88f628x" },
// ];
export default function Table({ columns, data }) {
    const tableInstance = useTable({ columns, data })
    const {
        getTableProps,
        getTableBodyProps,
        headerGroups,
        rows,
        prepareRow,
    } = tableInstance

    return (
        <div className="flex overflow-x-auto">
            <div className="w-full">
                <div className="bg-white shadow-md rounded my-6">
                    <table className="w-full table-auto" {...getTableProps()}>
                        <thead>
                            {
                                headerGroups.map(headerGroup => (
                                    <tr className="bg-gray-200 text-gray-600 uppercase text-sm leading-normal" {...headerGroup.getHeaderGroupProps()}>
                                        {
                                            headerGroup.headers.map(column => (
                                                <th className="py-3 px-6 text-left" {...column.getHeaderProps()}>{column.render('Header')}</th>
                                            ))
                                        }
                                    </tr>
                                ))
                            }
                        </thead>
                        <tbody className="text-gray-600 text-sm font-light" {...getTableBodyProps()}>
                            {rows.map(row => {
                                prepareRow(row)
                                return (
                                    <tr className="border-b border-gray-200 hover:bg-gray-100" {...row.getRowProps()}>
                                        {row.cells.map(cell => {
                                            return (
                                                <td className="py-3 px-6 text-left" {...cell.getCellProps()}>
                                                    {cell.render('Cell')}
                                                </td>
                                            )
                                        })}
                                    </tr>
                                )
                            })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
}
