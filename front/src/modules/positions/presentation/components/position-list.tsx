"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import Link from "next/link";

import type { Position } from "../../domain/position";

interface PositionListProps {
  positions: Position[];
}

export function PositionList({ positions }: PositionListProps) {
  if (positions.length === 0) {
    return (
      <div className="bg-muted/20 rounded-lg border p-8 text-center">
        <h2 className="text-xl font-semibold">No positions found</h2>
        <p className="text-muted-foreground">Start by applying to some jobs!</p>
      </div>
    );
  }

  return (
    <div className="grid gap-4">
      {positions.map((position) => (
        <Card key={position.id} className="transition-shadow hover:shadow-md">
          <CardHeader>
            <div className="flex items-start justify-between">
              <div className="flex flex-col gap-1">
                <CardTitle className="text-lg">{position.role_title}</CardTitle>
                <CardDescription>{position.company}</CardDescription>
              </div>
              <div className="bg-secondary text-secondary-foreground rounded-full px-2 py-1 text-xs font-semibold whitespace-nowrap">
                {position.status}
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="flex items-center justify-between gap-4">
              <div className="min-w-0 flex-1">
                {position.url && (
                  <a
                    href={position.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary truncate text-xs hover:underline"
                  >
                    {position.url}
                  </a>
                )}
              </div>
              <div className="text-muted-foreground shrink-0 text-xs">
                Applied on: {position.applied_on}
              </div>
            </div>
          </CardContent>
          <CardFooter>
            <Link href={`/dashboard/positions/${position.id}`} className="w-full">
              <Button variant="outline" size="sm" className="w-full">
                View Details
              </Button>
            </Link>
          </CardFooter>
        </Card>
      ))}
    </div>
  );
}
