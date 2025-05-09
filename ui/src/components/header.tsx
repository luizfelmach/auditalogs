export function Header() {
  return (
    <header className="border-b bg-white shadow-sm">
      <div className="container mx-auto py-4 px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-[#00BFB3] to-[#0077CC]">
              Audita Logs
            </h1>
          </div>
        </div>
      </div>
    </header>
  );
}
