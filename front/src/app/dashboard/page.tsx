import { PositionList } from "@/modules/positions/presentation/components/position-list";

export default function DashboardPage() {
    return (
        <div className="container mx-auto py-8">
            <div className="flex justify-between items-center mb-8">
                <h1 className="text-3xl font-bold">My Applications</h1>
            </div>
            <PositionList />
        </div>
    );
}
