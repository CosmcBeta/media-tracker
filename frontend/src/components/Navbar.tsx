import type React from "react";
import { Link } from "react-router";
import { cn } from "@/lib/utils";

interface NavbarProps extends React.ComponentProps<"div"> {}

function Navbar({ className }: NavbarProps) {
	return (
		<nav
			className={cn(
				"w-full h-16 sticky top-0 border-b select-none isolate z-50 bg-background text-foreground",
				className,
			)}
		>
			<div className="w-full h-full flex items-center justify-between py-2 px-6">
				<div className="flex items-center gap-10">
					<Logo />
				</div>
			</div>
		</nav>
	);
}

function Logo() {
	return (
		<Link to="/" className="text-xl font-bold text-primary tracking-tighter">
			Atlas
		</Link>
	);
}

export default Navbar;
