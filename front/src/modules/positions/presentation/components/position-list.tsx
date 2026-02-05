"use client";

import { useEffect, useState } from "react";
import { toast } from "sonner";
import { useRouter } from "next/navigation";

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";

import type { Position } from "../../domain/position";
import { positionService } from "../../composition-root";

export function PositionList() {
    const [positions, setPositions] = useState<Position[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const router = useRouter();

    useEffect(() => {
        async function fetchPositions() {
            try {
                const data = await positionService.getPositions();
                setPositions(data);
            } catch (error) {
                console.error(error);
                if (error instanceof Error && error.message === "Unauthorized") {
                    toast.error("Session expired", {
                        description: "Please log in again.",
                    });
                    router.push("/auth/login");
                } else {
                    toast.error("Error loading positions", {
                        description: "Please try again later.",
                    });
                }
            } finally {
                setIsLoading(false);
            }
        }

        fetchPositions();
    }, [router]);

    if (isLoading) {
        return (
            <div className="text-center p-8">Loading positions...</div>
        );
    }

    if (positions.length === 0) {
        return (
            <div className="text-center p-8">
                <h2 className="text-xl font-semibold">No positions found</h2>
                <p className="text-muted-foreground">Start by applying to some jobs!</p>
            </div>
        );
    }

    return (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {positions.map((position) => (
                <Card key={position.id}>
                    <CardHeader>
                        <div className="flex justify-between items-start">
                            <div className="flex flex-col gap-1">
                                <CardTitle>{position.role_title}</CardTitle>
                                <CardDescription>{position.company}</CardDescription>
                            </div>
                            <div className="px-2 py-1 rounded bg-secondary text-secondary-foreground text-xs font-semibold">
                                {position.status}
                            </div>
                        </div>
                    </CardHeader>
                    <CardContent>
                        <p className="text-sm line-clamp-3 mb-2">{position.description}</p>
                        <div className="text-xs text-muted-foreground mt-2">
                            Applied on: {position.applied_on}
                        </div>
                        {position.url && (
                            <a href={position.url} target="_blank" rel="noopener noreferrer" className="text-xs text-primary hover:underline mt-1 block">
                                View Job Post
                            </a>
                        )}
                    </CardContent>
                    <CardFooter>
                        {/* TODO: Add actions like Edit/Delete */}
                        <Button variant="outline" size="sm" className="w-full">View Details</Button>
                    </CardFooter>
                </Card>
            ))}
        </div>
    );
}
