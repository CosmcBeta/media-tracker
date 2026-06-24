import { Menu, X } from "lucide-react";
import {
	AnimatePresence,
	type MotionProps,
	motion,
	stagger,
} from "motion/react";
import type React from "react";
import { useState } from "react";
import { Link, useLocation } from "react-router";
import { cn } from "@/lib/utils";
import { Button } from "./ui/button";

const LINKS = [
	{ text: "Lists", url: "/lists" },
	{ text: "Items", url: "/items" },
	{ text: "Logbook", url: "#" },
];

const OVERLAY_ANIMATION: MotionProps = {
	initial: { opacity: 0 },
	animate: { opacity: 1 },
	exit: { opacity: 0, transition: { duration: 0.1 } },
};

// const BUTTONS_ANIMATION: MotionProps = {
// 	initial: { filter: "blur(4px)", opacity: 0, y: 20 },
// 	animate: { filter: "blur(0px)", opacity: 1, y: 0 },
// 	exit: { filter: "blur(4px)", opacity: 0, y: 20 },
// 	transition: {
// 		duration: 0.1 * LINKS.length,
// 		ease: [0.19, 1, 0.22, 1],
// 		delay: 0.7,
// 	},
// };

const ITEMS_CONTAINER_ANIMATION: MotionProps = {
	initial: "init",
	animate: "open",
	exit: "close",
	variants: {
		open: {
			transition: { delayChildren: stagger(0.07, { startDelay: 0.03 }) },
		},
		close: { transition: { delayChildren: stagger(0.05, { from: "last" }) } },
	},
};

const ITEMS_ANIMATION: MotionProps = {
	variants: {
		init: { filter: "blur(4px)", opacity: 0, y: 20 },
		open: { filter: "blur(0px)", opacity: 1, y: 0 },
		close: { filter: "blur(4px)", opacity: 0, y: 20 },
	},
	transition: {
		duration: 1.5,
		ease: [0.19, 1, 0.22, 1],
	},
};

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

					<DesktopNavbar />
				</div>

				<MobileNavbar />
			</div>
		</nav>
	);
}

function DesktopNavbar() {
	const location = useLocation();

	return (
		<div className="hidden lg:flex items-center gap-10">
			{}
			<ul className="flex">
				{LINKS.map((link) => (
					<li key={link.url}>
						<Link to={link.url}>
							<Button
								variant="ghost"
								size="sm"
								className={cn(
									location.pathname === link.url
										? "text-foreground"
										: "text-muted-foreground",
								)}
							>
								{link.text}
							</Button>
						</Link>
					</li>
				))}
			</ul>

			{/*<Actions />*/}
		</div>
	);
}

function MobileNavbar() {
	const [isOpen, setOpen] = useState<boolean>(false);
	const location = useLocation();

	const handleClick = () => {
		setOpen(!isOpen);
	};

	return (
		<div className="flex lg:hidden items-center gap-2">
			{/*<div className="hidden sm:block">
				<Actions />
			</div>*/}

			<AnimatePresence>
				{isOpen && (
					<motion.div
						{...OVERLAY_ANIMATION}
						className="fixed top-16 py-10 px-6 left-0 w-full h-[calc(100vh-4rem)] bg-background will-change-transform space-y-10 overflow-y-scroll no-scrollbar"
					>
						<motion.ul
							{...ITEMS_CONTAINER_ANIMATION}
							className="flex flex-col gap-4"
						>
							{LINKS.map((link) => (
								<motion.li key={link.url} {...ITEMS_ANIMATION}>
									<Link to={link.url} onClick={handleClick}>
										<Button
											variant="ghost"
											size="lg"
											className={cn(
												"text-xl",
												location.pathname === link.url
													? "text-foreground"
													: "text-muted-foreground",
											)}
										>
											{link.text}
										</Button>
									</Link>
								</motion.li>
							))}
						</motion.ul>

						{/*<motion.div {...BUTTONS_ANIMATION} className="block sm:hidden">
							<Actions />
						</motion.div>*/}
					</motion.div>
				)}
			</AnimatePresence>

			<Button size="icon-lg" variant="ghost" onClick={handleClick}>
				{isOpen ? <X /> : <Menu />}
			</Button>
		</div>
	);
}

// function Actions() {
// 	return (
// 		<div className="flex items-center gap-2 px-0">
// 			<Link to="#">
// 				<Button variant="ghost" className="text-muted-foreground">
// 					Log in
// 				</Button>
// 			</Link>

// 			<Link to="#">
// 				<Button>Sign up</Button>
// 			</Link>
// 		</div>
// 	);
// }

function Logo() {
	return (
		<Link to="#" className="text-xl font-bold text-primary tracking-tighter">
			Atlas
		</Link>
	);
}

export default Navbar;
