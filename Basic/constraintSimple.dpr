program constraintSimple;

uses
  System.StartUpCopy,
  FMX.Forms,
  constraintSimple.fmain in 'constraintSimple.fmain.pas' {FMain};

{$R *.res}

begin
  {$IFDEF DEBUG}
  ReportMemoryLeaksOnShutdown := true;
  {$ENDIF}
  Application.Initialize;
  Application.CreateForm(TFMain, FMain);
  Application.Run;
end.
