/*
 * TRACE32 Remote API
 *
 * Copyright (c) 1998-2020 Lauterbach GmbH
 * All rights reserved
 *
 * Licensing restrictions apply to this code.
 * Please see documentation (api_remote_c.pdf) for
 * licensing terms and conditions.
 *
 * $LastChangedRevision: 178374 $
 */


#ifndef T32_H
#define T32_H
//#define ENABLE_NOTIFICATION

/************************/
/* standard definitions */
/************************/


#ifndef UINT32_MAX
# if defined(_MSC_VER) && (_MSC_VER<1600)
#  define  UINT32_MAX        4294967295U
typedef __int8            int8_t;
typedef __int16           int16_t;
typedef __int32           int32_t;
typedef __int64           int64_t;
typedef unsigned __int8   uint8_t;
typedef unsigned __int16  uint16_t;
typedef unsigned __int32  uint32_t;
typedef unsigned __int64  uint64_t;
# else
#  if defined(__SUNPRO_C) || defined(__SUNPRO_CC)
#   include <inttypes.h>
#  else
#   include <stdint.h>
#  endif
# endif
#endif

#if defined(__GNUC__) || defined(__clang__)
# define ATTRIBUTE_PRINTF(FORMATPOS,FIRSTARGPOS) __attribute__((format (__printf__, FORMATPOS, FIRSTARGPOS)))
#else
# define ATTRIBUTE_PRINTF(FORMATPOS,FIRSTARGPOS)
#endif

#ifdef DLL_BUILD
# define T32EXTERN __declspec(dllexport)
#else
# define T32EXTERN extern
#endif

#ifndef _NO_PROTO
# ifdef __cplusplus
extern "C" {
# endif

#ifdef ENABLE_THREADLOCALAPI
# if defined(__GNUC__) || defined(__clang__)
#  define T32_THREADLOCAL __thread
# else
#  define T32_THREADLOCAL __declspec(thread)
# endif
#else
# define T32_THREADLOCAL
#endif

#define T32_PCKLEN_MAX          1472


/************************/
/* returned error codes */
/************************/

#define T32_OK                          0
/* error codes by API client */
#define T32_ERR_COM_RECEIVE_FAIL        -1      /* receiving API response failed */
#define T32_ERR_COM_TRANSMIT_FAIL       -2      /* sending API message failed */
#define T32_ERR_COM_PARA_FAIL           -3      /* function parameter error */
#define T32_ERR_COM_SEQ_FAIL            -4      /* message sequence failed */
#define T32_ERR_NOTIFY_MAX_EVENT        -5      /* max. notify events exceeded */
#define T32_ERR_MALLOC_FAIL             -6      /* malloc() failed */
/* standard error codes */
#define T32_ERR_STD_GENERIC             1       /* generic error (do not check for this error code) */
#define T32_ERR_STD_RUNNING             2       /* target running */
#define T32_ERR_STD_NOTRUNNING          3       /* target not running */
#define T32_ERR_STD_RESET               4       /* target is in reset */
#define T32_ERR_STD_ACCESSTIMEOUT       6       /* access timeout, target running */
#define T32_ERR_STD_INVALID             10      /* not implemented */
#define T32_ERR_STD_REGUNDEF            14      /* registerset undefined */
#define T32_ERR_STD_VERIFY              15      /* verify error */
#define T32_ERR_STD_BUSERROR            16      /* bus error */
#define T32_ERR_STD_NOMEM               22      /* no memory mapped */
#define T32_ERR_STD_RESETDETECTED       48      /* target reset detected */
#define T32_ERR_STD_FDXBUFFER           49      /* FDX buffer error */
#define T32_ERR_STD_RTCKTIMEOUT         57      /* no RTCK detected */
#define T32_ERR_STD_INVALIDLICENSE      60      /* no valid license detected */
#define T32_ERR_STD_CORENOTACTIVE       64      /* core has no clock/power/reset in SMP */
#define T32_ERR_STD_USERSIGNAL          67      /* user signal */
#define T32_ERR_STD_NORAPI              83      /* tried to connect to emu */
#define T32_ERR_STD_FAILED              113
#define T32_ERR_STD_LOCKED              123     /* access locked */
#define T32_ERR_STD_POWERFAIL           128     /* power fail */
#define T32_ERR_STD_DEBUGPORTFAIL       140     /* debug port fail */
#define T32_ERR_STD_DEBUGPORTTIMEOUT    144     /* debug port timeout */
#define T32_ERR_STD_NODEVICE            147     /* no debug device */
#define T32_ERR_STD_RESETFAIL           161     /* target reset fail */
#define T32_ERR_STD_EMUTIMEOUT          162     /* emulator communication timeout */
#define T32_ERR_STD_NORTCK              164     /* no RTCK on emulator */
#define T32_ERR_STD_ATTACH              254     /* T32_Attach() is missing */
#define T32_ERR_STD_FATAL               255

#define T32_ERR_GETRAM_INTERNAL                  0x1000  /* T32_GetRam failed internally */
#define T32_ERR_READREGBYNAME_NOTFOUND           0x1010  /* T32_ReadRegisterByName: register not found */
#define T32_ERR_READREGBYNAME_FAILED             0x1011  /* T32_ReadRegisterByName: reading register failed */
#define T32_ERR_WRITEREGBYNAME_NOTFOUND          0x1020  /* T32_WriteRegisterByName: register not found */
#define T32_ERR_WRITEREGBYNAME_FAILED            0x1021  /* T32_WriteRegisterByName: reading register failed */
#define T32_ERR_READREGOBJ_PARAFAIL              0x1030  /* T32_ReadRegisterObj: wrong parameters */
#define T32_ERR_READREGOBJ_MAXCORE               0x1031  /* T32_ReadRegisterObj: max cores exceeded */
#define T32_ERR_READREGOBJ_NOTFOUND              0x1032  /* T32_ReadRegisterObj: register not found */
#define T32_ERR_READREGSETOBJ_PARAFAIL           0x1033  /* T32_ReadRegisterSetObj: wrong parameters */
#define T32_ERR_READREGSETOBJ_NUMREGS            0x1034  /* T32_ReadRegisterSetObj: number of read registers wrong */
#define T32_ERR_WRITEREGOBJ_PARAFAIL             0x1040  /* T32_WriteRegisterObj: wrong parameters */
#define T32_ERR_WRITEREGOBJ_MAXCORE              0x1041  /* T32_WriteRegisterObj: max cores exceeded */
#define T32_ERR_WRITEREGOBJ_NOTFOUND             0x1042  /* T32_WriteRegisterObj: register not found */
#define T32_ERR_WRITEREGOBJ_FAILED               0x1043  /* T32_WriteRegisterObj: writing register failed */
#define T32_ERR_SETBP_FAILED                     0x1050  /* T32_WriteBreakpoint/T32_WriteBreakpointObj: setting breakpoint failed */
#define T32_ERR_READMEMOBJ_PARAFAIL              0x1060  /* T32_ReadMemoryObj: wrong parameters */
#define T32_ERR_WRITEMEMOBJ_PARAFAIL             0x1070  /* T32_WriteMemoryObj: wrong parameters */
#define T32_ERR_TRANSFERMEMOBJ_PARAFAIL          0x1071  /* T32_TransferMemoryBundleObj: wrong parameters */
#define T32_ERR_TRANSFERMEMOBJ_TRANSFERFAIL      0x1072  /* T32_TransferMemoryBundleObj: transfer failed */
#define T32_ERR_READVAR_ALLOC                    0x1080  /* T32_ReadVariable*: mem alloc failed */
#define T32_ERR_READVAR_ACCESS                   0x1081  /* T32_ReadVariable*: access to symbol failed */
#define T32_ERR_READBPOBJ_PARAFAIL               0x1091  /* T32_ReadBreakpointObj: wrong parameters */
#define T32_ERR_READBPOBJ_NOTFOUND               0x1092  /* T32_ReadBreakpointObj: breakpoint not found */
#define T32_ERR_WRITEBPOBJ_FAILED                0x10a1  /* T32_WriteBreakpointObj: setting BP failed */
#define T32_ERR_WRITEBPOBJ_ADDRESS               0x10a2  /* T32_WriteBreakpointObj: address error */
#define T32_ERR_WRITEBPOBJ_ACTION                0x10a3  /* T32_WriteBreakpointObj: action error */
#define T32_ERR_MMUTRANSLATION_FAIL              0x10b0  /* T32_QueryAddressObjMmuTranslation: translation failed */
#define T32_ERR_EXECUTECOMMAND_FAIL              0x10c0  /* T32_ExecuteCommand: command failed */
#define T32_ERR_EXECUTEFUNCTION_FAIL             0x10c1  /* T32_ExecuteFunction: function failed */
/* legacy definitions */
#define T32_COM_RECEIVE_FAIL    T32_ERR_COM_RECEIVE_FAIL
#define T32_COM_TRANSMIT_FAIL   T32_ERR_COM_TRANSMIT_FAIL
#define T32_COM_PARA_FAIL       T32_ERR_COM_PARA_FAIL
#define T32_COM_SEQ_FAIL        T32_ERR_COM_SEQ_FAIL
#define T32_MAX_EVENT_FAIL      T32_ERR_NOTIFY_MAX_EVENT
#define T32_MALLOC_FAIL         T32_ERR_MALLOC_FAIL


/***********************/
/* basic API functions */
/***********************/

#define T32_DEV_OS              0
#define T32_DEV_ICE             1
#define T32_DEV_ICD             1

T32EXTERN T32_THREADLOCAL int T32_Errno;
T32EXTERN int  T32_Config(const char *String1, const char *String2);
T32EXTERN int  T32_Init(void);
T32EXTERN int  T32_Attach(int DeviceSpecifier);
T32EXTERN int  T32_Terminate(int ShellReturnValue);
T32EXTERN int  T32_Exit(void);
T32EXTERN int  T32_Ping(void);
T32EXTERN int  T32_Nop(void);
T32EXTERN int  T32_NopEx(int length, int options);
T32EXTERN int  T32_NopFail(void);
T32EXTERN int  T32_Cmd(const char *Command);
T32EXTERN int  T32_Cmd_f(const char *command, ...) ATTRIBUTE_PRINTF(1, 2);
T32EXTERN int  T32_CmdWin(uint32_t WindowHandle, const char *Command);
T32EXTERN int  T32_ExecuteCommand(const char *pCommand, char* pBuffer, uint32_t BufferSize);
T32EXTERN int  T32_ExecuteFunction(const char *pExpression, char* pBuffer, uint32_t BufferSize, uint32_t *pResultType);
T32EXTERN int  T32_ExecuteFunction_Double(const char *pExpression, char* pBuffer, uint32_t BufferSize, double *pResult);
T32EXTERN int  T32_ExecuteFunction_UInt64(const char *pExpression, char* pBuffer, uint32_t BufferSize, uint64_t *pResult);
T32EXTERN int  T32_Printf(const char *str, ...) ATTRIBUTE_PRINTF(1, 2);
T32EXTERN int  T32_Stop(void);
T32EXTERN int  T32_GetPracticeState(int *pPracticeState);
T32EXTERN int  T32_EvalGet(uint32_t *pEvaluationResult);
T32EXTERN int  T32_EvalGetString(char *EvaluationString);
T32EXTERN int  T32_GetMessage(char *AreaMessage, uint16_t *pMessageType);
T32EXTERN int  T32_GetMessageString(char* AreaMessage, uint16_t ArraySize, uint16_t *pMessageType, uint16_t *pMessageLen);
T32EXTERN int  T32_GetTriggerMessage(char *TriggerMessage);
T32EXTERN int  T32_GetChannelSize(void);
T32EXTERN int  T32_GetChannelSizeNetTcp(void);  /* preliminary */
T32EXTERN void T32_GetChannelDefaults(void *pChannel);
T32EXTERN void T32_GetChannelDefaultsNetTcp(void *pChannel);  /* preliminary */
T32EXTERN int  T32_RequestChannelNetAssist(void **ppChannel);
T32EXTERN int  T32_RequestChannelNetTcp(void **ppChannel);
T32EXTERN int  T32_ReleaseChannel(void **gChannel);
T32EXTERN void T32_SetChannel(void *ParametersIn);
T32EXTERN int  T32_APILock(int nTimeoutMS);  /* preliminary*/
T32EXTERN int  T32_APIUnlock(void);
T32EXTERN int  T32_GetApiRevision(uint32_t* pRevNum);
T32EXTERN void T32_GetSocketHandle(int *t32soc);


/*******************************/
/* debug related API functions */
/*******************************/


#define T32_MEMORY_ACCESS_DATA                   0x0000
#define T32_MEMORY_ACCESS_PROGRAM                0x0001
#define T32_MEMORY_ACCESS_USR                    0x000f
#define T32_MEMORY_ACCESS_VM                     0x0010
#define T32_MEMORY_ACCESS_ARM_CP0                0x0002
#define T32_MEMORY_ACCESS_ARM_ICE                0x0003
#define T32_MEMORY_ACCESS_ARM_ETM                0x0004
#define T32_MEMORY_ACCESS_ARM_CP14               0x0005
#define T32_MEMORY_ACCESS_ARM_CP15               0x0006
#define T32_MEMORY_ACCESS_ARM_ARM                0x0007
#define T32_MEMORY_ACCESS_ARM_THUMB              0x0008
#define T32_MEMORY_ACCESS_ARM_PHYSICAL_ARM       0x0009
#define T32_MEMORY_ACCESS_ARM_PHYSICAL_THUMB     0x000a
#define T32_MEMORY_ACCESS_ARM_ETB                0x000b
#define T32_MEMORY_ACCESS_ARM_PHYSICAL_DATA      0x000c
#define T32_MEMORY_ACCESS_ARM_PHYSICAL_PROGRAM   0x000d
#define T32_MEMORY_ACCESS_ARM_DAP                0x000e
#define T32_MEMORY_ACCESS_ARM_USR                T32_MEMORY_ACCESS_USR
#define T32_MEMORY_ACCESS_ARM_DBG                0x0011
#define T32_MEMORY_ACCESS_ARC_AUX                0x0002
#define T32_MEMORY_ACCESS_PPC_SPR                0x0002
#define T32_MEMORY_ACCESS_PPC_DCR                0x0003
#define T32_MEMORY_ACCESS_X86_LD                 0x0002
#define T32_MEMORY_ACCESS_X86_LP                 0x0003
#define T32_MEMORY_ACCESS_X86_IO                 0x0004
#define T32_MEMORY_ACCESS_X86_MSR                0x0005
#define T32_MEMORY_ACCESS_XTENSA_DAP             T32_MEMORY_ACCESS_ARM_DAP
#define T32_MEMORY_ACCESS_XTENSA_SPR             0x0002
#define T32_MEMORY_ACCESS_MMUBYPASS              0x8000
#define T32_MEMORY_ATTR_WIDTHMASK                0x000f
#define T32_MEMORY_ATTR_DUALPORT                 0x0400
#define T32_MEMORY_ATTR_NOINCREMENT              0x4000

#define T32_MAX_EVENTS                                7
#define T32_E_BREAK                                0x00
#define T32_E_EDIT                                 0x01
#define T32_E_BREAKPOINTCONFIG                     0x02
#define T32_E_ONEVENT                              0x03
#define T32_E_RTSTRIGGER                           0x04
#define T32_E_ERROR                                0x05
#define T32_E_GO                                   0x06
#define T32_E_BREAKSYNC_ENABLE                     0x03 /* Eclipse only */
#define T32_E_BREAKSYNC_DISABLE                    0x04 /* Eclipse only */

/* Notification codes for T32_E_BREAK */
#define T32_BREAKNOTIFICATIONCODE_TYPEMASK        0xFF000000 /* generic information about breakpoint type */
#define T32_BREAKNOTIFICATIONCODE_TYPE_GENERIC    0x00000000 /* manual halt or undefined reason */
#define T32_BREAKNOTIFICATIONCODE_TYPE_SOFT       0x01000000 /* software breakpoint */
#define T32_BREAKNOTIFICATIONCODE_TYPE_ONCHIP     0x02000000 /* onchip breakpoint */

#define T32_BREAKNOTIFICATIONCODE_RESOURCEMASK    0x00FF0000 /* used resource (optional) */
#define T32_BREAKNOTIFICATIONCODE_RES_OTHER       0x00000000 /* other resource */
#define T32_BREAKNOTIFICATIONCODE_RES_INSTRUCTION 0x00010000 /* instruction breakpoint */
#define T32_BREAKNOTIFICATIONCODE_RES_DATA        0x00020000 /* data address breakpoint */
#define T32_BREAKNOTIFICATIONCODE_RES_COUNTER     0x00030000 /* counter breakpoint */
#define T32_BREAKNOTIFICATIONCODE_RES_TRIGGER     0x00040000 /* trigger input of processor */

#define T32_BREAKNOTIFICATIONCODE_IDMASK          0x0000FFFF /* resource ID (optional, processor specific) */

/* Notification codes for T32_E_RTSTRIGGER */
#define T32_RTSTRIGGERNOTIFICATIONCODE_EXECUTE 0x02 /* RTS.TriggerOnExecute */
#define T32_RTSTRIGGERNOTIFICATIONCODE_READ    0x04 /* RTS.TriggerOnRead */
#define T32_RTSTRIGGERNOTIFICATIONCODE_WRITE   0x06 /* RTS.TriggerOnWrite */
#define T32_RTSTRIGGERNOTIFICATIONCODE_WTM     0x08 /* RTS.TriggerOnWTM */
#define T32_RTSTRIGGERNOTIFICATIONCODE_ERROR   0x0A /* RTS streaming error */

/* T32_RTSTRIGGERNOTIFICATIONCODE_ERROR: Error types (param) */
#define T32_RTSTRIGGERERROR_PARAM_FIFOFULL                       1 /* the target signalled a trace FIFO overflow */
#define T32_RTSTRIGGERERROR_PARAM_OUTOFMEMORY                    2 /* out of memory */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_ILLEGALCANCEL             3 /* flow error: illegal cancel packet */
#define T32_RTSTRIGGERERROR_PARAM_BADPACKET                      4 /* bad trace packet */
#define T32_RTSTRIGGERERROR_PARAM_FLOW                           5 /* flow error: generic error */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_UNEXPECTEDINDIRECTBRANCH  6 /* flow error: unexpected indirect branch */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_UNEXPECTEDADDRESS         7 /* flow error: unexpected address */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_UNEXPECTEDDIRECTBRANCH    8 /* flow error: unexpected direct branch */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_NOINDIRECTBRANCH          9 /* flow error: missing indirect branch */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_TOOMANYSEQ               10 /* flow error: too many sequential instructions */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_SPECQUEUE                11 /* flow error: spec queue mismatch */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_CYCLEQUEUE               12 /* flow error: cycle queue mismatch */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_CONFIG                   13 /* flow error: configuration mismatch */
#define T32_RTSTRIGGERERROR_PARAM_NOACCESSTOCODE                14 /* no access to code */
#define T32_RTSTRIGGERERROR_PARAM_VTRACE                        15 /* vtrace error */
#define T32_RTSTRIGGERERROR_PARAM_CORESIGHT                     16 /* predecoder error */
#define T32_RTSTRIGGERERROR_PARAM_BADADDRESS                    17 /* bad data address */
#define T32_RTSTRIGGERERROR_PARAM_TRIGGEROVERFLOW               18 /* trigger FIFO overflow */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_BADICOUNT                19 /* flow error: bad instruction count */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_NODIRECTBRANCH           20 /* flow error: missing direct branch */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_BADTIMESTAMP             21 /* flow error: bad timestamp */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_UNEXPECTED_IPI0          22 /* flow error: unexpected IPI0 message */
#define T32_RTSTRIGGERERROR_PARAM_FLOW_UNEXPECTED_IPA           23 /* flow error: unexpected IPA message */
#define T32_RTSTRIGGERERROR_PARAM_TRIGGERERROR                  24 /* trigger error */
#define T32_RTSTRIGGERERROR_PARAM_OVERFLOW                      25 /* trace FIFO on trace tool overflowed */

/* Macros used for Remote API communication layer */
#define T32_API_RECEIVE                            0x01
#define T32_API_SYNCREQUEST                        0x02
#define T32_API_CONNECTREQUEST                     0x03
#define T32_API_DISCONNECT                         0x04
#define T32_API_NOTIFICATION                       0x06
#define T32_API_TRANSMIT                           0x11
#define T32_API_SYNCACKN                           0x12
#define T32_API_SYNCACKN_BACK                      0x13
#define T32_API_SYNCBACK                           0x22

/* Flags for T32_WriteBreakpoint() / T32_ReadBreakpoint */
#define T32_BPCONFIG_PROGRAM                      0x001 /* program breakpoint / execution breakpoint */
#define T32_BPCONFIG_SPOT                         0x004 /* spot breakpoint */
#define T32_BPCONFIG_READ                         0x008 /* read access breakpoint  */
#define T32_BPCONFIG_WRITE                        0x010 /* write access breakpoint */
#define T32_BPCONFIG_READWRITE                    0x018 /* read/write access breakpoint */
#define T32_BPCONFIG_DELETE                       0x100 /* delete breakpoint */
#define T32_BPCONFIG_ALPHA                        0x020 /* Alpha  breakpoint or marker */
#define T32_BPCONFIG_BETA                         0x040 /* Beta   breakpoint or marker */
#define T32_BPCONFIG_CHARLIE                      0x080 /* Charlie breakpoint or marker */

#define T32_BPACCESS_DELTA                         0x10
#define T32_BPACCESS_ECHO                          0x20
#define T32_BPACCESS_E                             0x40
#define T32_BPACCESS_ONCHIP                        0x80

#define T32_PRINTCODE_ASCII                        0x41
#define T32_PRINTCODE_ASCIIP                       0x42
#define T32_PRINTCODE_ASCIIE                       0x43
#define T32_PRINTCODE_CSV                          0x44
#define T32_PRINTCODE_XML                          0x45

typedef struct {
	uint32_t  address;
	uint8_t   enabled;
	uint32_t  type;
	uint32_t  auxtype;
} T32_Breakpoint;

T32EXTERN int  T32_Go(void);
T32EXTERN int  T32_Break(void);
T32EXTERN int  T32_Step(void);
T32EXTERN int  T32_StepMode(int Mode);
T32EXTERN int  T32_ResetCPU(void);
T32EXTERN int  T32_SetMode(int Mode);
T32EXTERN int  T32_GetCpuInfo(char **pCPUString, uint16_t *pFPUType, uint16_t *pEndianess, uint16_t *pReserved);
T32EXTERN int  T32_GetState(int *pSystemState);
T32EXTERN int  T32_ReadMemory     (uint32_t Address, int Access, uint8_t *pBuffer, int Size);
T32EXTERN int  T32_WriteMemory    (uint32_t Address, int Access, const uint8_t *pBuffer, int Size);
T32EXTERN int  T32_WriteMemoryPipe(uint32_t Address, int Access, const uint8_t *pBuffer, int Size);
T32EXTERN int  T32_SetMemoryAccessClass(const char* Access);
T32EXTERN int  T32_GetRam(uint32_t *pStartAddress, uint32_t *pEndAddress, uint16_t *pAccess);
T32EXTERN int  T32_GetSource(uint32_t Address, char *SourceFile, uint32_t *pSourceLine);
T32EXTERN int  T32_GetSelectedSource(          char *SourceFile, uint32_t *pSourceLine);
T32EXTERN int  T32_GetSymbol(const char *SymbolName, uint32_t *pAddress, uint32_t *pSize, uint32_t *pAccess);
T32EXTERN int  T32_GetSymbolFromAddress (char* SymbolName, uint32_t Address, int StringLength);
T32EXTERN int  T32_ReadVariableString(const char *VariableName, char *StringFromNumeric, int StringLength);
T32EXTERN int  T32_ReadVariableValue (const char *VariableName, uint32_t *pValueLower32Bit, uint32_t *pValueUpper32Bit);
T32EXTERN int  T32_WriteVariableValue (const char *VariableName, uint32_t ValueLower32Bit, uint32_t ValueUpper32Bit);
T32EXTERN int  T32_GetWindowContent(const char *command, char * buffer, uint32_t requested, uint32_t offset, uint32_t print_code);
T32EXTERN int  T32_ReadRegisterByName (const char *RegisterName, uint32_t *pValueLower32Bit, uint32_t *pValueUpper32Bit);
T32EXTERN int  T32_WriteRegisterByName(const char *RegisterName, uint32_t ValueLower32Bit, uint32_t ValueUpper32Bit);
T32EXTERN int  T32_ReadPP(uint32_t *pProgramCounter);
T32EXTERN int  T32_ReadRegister (uint32_t MaskLower32Bit, uint32_t MaskUpper32Bit, uint32_t *pBuffer);
T32EXTERN int  T32_WriteRegister(uint32_t MaskLower32Bit, uint32_t MaskUpper32Bit, uint32_t *pBuffer);
T32EXTERN int  T32_ReadBreakpoint (uint32_t Address, int Access, uint16_t *pBPConfiguration, int Size);
T32EXTERN int  T32_WriteBreakpoint(uint32_t Address, int Access, int        BPConfiguration, int Size);
T32EXTERN int  T32_GetBreakpointList(int *pNumberFetched, T32_Breakpoint *pBPSettings, int FetchLimit);
T32EXTERN int  T32_GetTraceState(int TraceType, int *pTraceState, int32_t *pTraceTotalRecords, int32_t *pCurrentRecordMin, int32_t *pCurrentRecordMax);
T32EXTERN int  T32_ReadTrace    (int TraceType, int32_t StartRecord, int NumberOfRecords, uint32_t Mask, uint8_t *pBuffer);
T32EXTERN int  T32_GetLastErrorMessage(char *ErrorMessage, uint32_t* pLastError, uint32_t* pInternal);
# ifdef ENABLE_NOTIFICATION
typedef void (*T32_NotificationCallback_t)();
T32EXTERN int T32_CheckStateNotify(unsigned ParameterOfCallbackFunction);
T32EXTERN int T32_NotificationPending(void);
typedef void (*T32_NotificationCallback_i_t)    (int);
typedef void (*T32_NotificationCallback_iqq_t)  (int, uint64_t, uint64_t);
typedef void (*T32_NotificationCallback_iicp_t) (int, int, const char*);
typedef void (*T32_NotificationCallback_iqqq_t) (int, uint64_t, uint64_t, uint64_t);
T32EXTERN int T32_NotifyEventEnable(const char* event, T32_NotificationCallback_i_t pFunction);
T32EXTERN int T32_NotifyStateEnable(int EventNumber, T32_NotificationCallback_t pFunction);
inline int T32_NotifyBreakEnable(T32_NotificationCallback_iqq_t pFunction){
	return T32_NotifyStateEnable(T32_E_BREAK, (T32_NotificationCallback_t) pFunction);
}
inline int T32_NotifyEditEnable(T32_NotificationCallback_iicp_t pFunction){
	return T32_NotifyStateEnable(T32_E_EDIT, (T32_NotificationCallback_t) pFunction);
}
inline int T32_NotifyBreakConfigEnable(T32_NotificationCallback_i_t pFunction){
	return T32_NotifyStateEnable(T32_E_BREAKPOINTCONFIG, (T32_NotificationCallback_t) pFunction);
}
inline int T32_NotifyRTSTriggerEnable(T32_NotificationCallback_iqqq_t pFunction){
	return T32_NotifyStateEnable(T32_E_RTSTRIGGER, (T32_NotificationCallback_t) pFunction);
}
inline int T32_NotifyGoEnable(T32_NotificationCallback_i_t pFunction){
	return T32_NotifyStateEnable(T32_E_GO, (T32_NotificationCallback_t) pFunction);
}
inline int T32_NotifyErrorEnable(T32_NotificationCallback_iicp_t pFunction){
	return T32_NotifyStateEnable(T32_E_ERROR, (T32_NotificationCallback_t) pFunction);
}
# endif
/*** UNDOCUMENTED (no support) ************************************************/
T32EXTERN int  T32_ReadMemoryEx (uint32_t Address, int Segment, int Access, int Attribute, uint8_t *pBuffer, int Size);
T32EXTERN int  T32_WriteMemoryEx (uint32_t Address, int Segment, int Access, int Attribute, uint8_t *pBuffer, int Size);
T32EXTERN int  T32_FlashfileReadMemory (uint64_t Address, uint8_t *pBuffer, int Size);
T32EXTERN int  T32_FlashfileWriteMemory (uint64_t Address, const uint8_t *pBuffer, int Size);
T32EXTERN int  T32_FlashfileFlushMemory (uint64_t Address, int Size, uint32_t *pFlushResult);
/*** DEPRECATED ***************************************************************/
T32EXTERN int T32_AnaStatusGet(uint8_t * state, int32_t * size, int32_t * min, int32_t * max);
T32EXTERN int T32_AnaRecordGet(int32_t record, uint8_t * buffer, int len);
/******************************************************************************/


/***********************************/
/*  Object oriented API functions  */
/***********************************/

/*** Obj Interface naming convention
<objtype>:                  Buffer, Address, Register, Memory, Symbol
Object types:               T32_<objtype>Obj
Object handle types:        T32_<objtype>Handle
Allocating objects:         T32_Request<objtype>Obj
//                          T32_Request<objtype>Obj<initial>
Reallocating objects:       T32_Resize<objtype>Obj
Freeing objects:            T32_Release<objtype>Obj
//                          T32_ReleaseAllObjects
Getting object attributes:  T32_Get<objtype>Obj<attribute>
Setting object attributes:  T32_Set<objtype>Obj<attribute>
Copying existing object:    T32_Copy<objtype>Obj
Copying from/into objects:  T32_Copy<what>From/To<objtype>Obj
Reading from target:        T32_Read<objtype>Obj
Reading by signifier:       T32_Read<objtype>ObjBy<signifier>
Writing to target:          T32_Write<objtype>Obj
Getting info from TRACE32:  T32_Query<objtype>Obj
Sending info to TRACE32:    T32_Send<objtype>Obj
***/

typedef struct sT32_Object T32_Object;
struct sT32_Object {
	T32_Object  *next;
};

T32EXTERN int T32_ReleaseAllObjects (void);

/** Buffer Object **/

typedef struct {
	T32_Object     *next;
	uint32_t        used;
	uint8_t        *storage;
	uint32_t        bufsize;
} T32_BufferObj;

/* Buffer handle */
typedef  T32_BufferObj* T32_BufferHandle;

/* request buffer object */
T32EXTERN int T32_RequestBufferObj (T32_BufferHandle *pHandle, const int initial_size);
T32EXTERN int T32_ReleaseBufferObj (T32_BufferHandle *pHandle);
T32EXTERN int T32_ResizeBufferObj  (T32_BufferHandle handle, const int size);

/* buffer access functions */
T32EXTERN int T32_CopyDataFromBufferObj (uint8_t *localbuffer, int lbsize, T32_BufferHandle handle);
T32EXTERN int T32_CopyDataToBufferObj   (T32_BufferHandle handle, int size, const uint8_t *localbuffer);
T32EXTERN int T32_CopyDataToMaskedBufferObj (T32_BufferHandle handle, int size, const uint8_t *localbuffer, const uint8_t *localmaskbuffer);
T32EXTERN int T32_GetBufferObjStoragePointer (uint8_t** ppointer, T32_BufferHandle handle);

/** Address Object **/

/* enumeration for all address types */
typedef enum {
	T32_ADDRTYPE_NONE,
	T32_ADDRTYPE_COMMON,
	T32_ADDRTYPE_A32,
	T32_ADDRTYPE_A64,
	T32_ADDRTYPE_MAX
} T32_AddressObjType;

/* memory access attributes */
#define T32_ADDROBJATTR_EACCESS  0x1
#define T32_ADDROBJATTR_VERIFY   0x2
#define T32_ADDROBJATTR_RTSTRIGGERACK 0x4
#define T32_ADDROBJATTR_GO       0x8
#define T32_ADDROBJATTR_BREAK    0x10

/* enumeration for MAU sizes (minimum addressable unit) */
typedef enum {
	T32_SIZEOFMAU_NOTSET    = 0,
	T32_SIZEOFMAU_TARGET    = 1,
	T32_SIZEOFMAU_8BIT      = 8,
	T32_SIZEOFMAU_12BIT     = 12,
	T32_SIZEOFMAU_16BIT     = 16,
	T32_SIZEOFMAU_24BIT     = 24,
	T32_SIZEOFMAU_32BIT     = 32,
	T32_SIZEOFMAU_64BIT     = 64,
	T32_SIZEOFMAU_128BIT    = 128,
	T32_SIZEOFMAU_256BIT    = 256
} T32_SizeOfMauType;

/* common address elements */
typedef struct {
	T32_Object         *next;
	T32_AddressObjType  type;
	char                access[16];
	uint16_t            width;
	uint16_t            core;
	uint32_t            machineid;
	uint32_t            spaceid;
	uint32_t            attr;
	T32_SizeOfMauType   sizeofmau;
	T32_SizeOfMauType   targetsizeofmau;
} T32_AddressObj_Common;

/* address type: 32bit address */
typedef struct {
	T32_AddressObj_Common common;
	uint32_t    address;
} T32_AddressObj_32bit;

/* address type: 64bit address */
typedef struct {
	T32_AddressObj_Common common;
	uint64_t address;
} T32_AddressObj_64bit;

/* address handle for all address types */
typedef union {
	T32_AddressObj_Common   common;
	T32_AddressObj_32bit    a32;
	T32_AddressObj_64bit    a64;
} T32_AddressObj;

/* Address handle */
typedef T32_AddressObj* T32_AddressHandle;

/* request address object */
T32EXTERN int T32_RequestAddressObj     (T32_AddressHandle *pHandle);
T32EXTERN int T32_RequestAddressObjA32  (T32_AddressHandle *pHandle, const uint32_t address);
T32EXTERN int T32_RequestAddressObjA64  (T32_AddressHandle *pHandle, const uint64_t address);
T32EXTERN int T32_ReleaseAddressObj     (T32_AddressHandle *pHandle);
T32EXTERN int T32_CopyAddressObj        (T32_AddressHandle *pToHandle, T32_AddressHandle fromHandle);

T32EXTERN int T32_SetAddressObjAddr32       (T32_AddressHandle handle, uint32_t address);
T32EXTERN int T32_GetAddressObjAddr32       (T32_AddressHandle handle, uint32_t *pAddress);
T32EXTERN int T32_SetAddressObjAddr64       (T32_AddressHandle handle, uint64_t address);
T32EXTERN int T32_GetAddressObjAddr64       (T32_AddressHandle handle, uint64_t *pAddress);
T32EXTERN int T32_SetAddressObjAccessString (T32_AddressHandle handle, const char *accessString);
T32EXTERN int T32_GetAddressObjAccessString (T32_AddressHandle handle, char* accessString, uint8_t maxlen);

T32EXTERN int T32_SetAddressObjWidth            (T32_AddressHandle handle, uint16_t width);
T32EXTERN int T32_SetAddressObjCore             (T32_AddressHandle handle, uint16_t core);
T32EXTERN int T32_SetAddressObjMachineId        (T32_AddressHandle handle, uint32_t machineid);
T32EXTERN int T32_SetAddressObjSpaceId          (T32_AddressHandle handle, uint32_t spaceid);
T32EXTERN int T32_SetAddressObjAttr             (T32_AddressHandle handle, uint32_t attributes);
T32EXTERN int T32_SetAddressObjSizeOfMau        (T32_AddressHandle handle, T32_SizeOfMauType sizeofmau);
T32EXTERN int T32_GetAddressObjSizeOfMau        (T32_AddressHandle handle, T32_SizeOfMauType* pSizeOfMau);
T32EXTERN int T32_GetAddressObjTargetSizeOfMau  (T32_AddressHandle handle, T32_SizeOfMauType* pTargetSizeOfMau);


#define T32_MMUTRANSLATION_TO_PHYSICAL  0x1
#define T32_MMUTRANSLATION_TO_LOGICAL   0x2
#define T32_MMUTRANSLATION_TO_LINEAR    0x3
T32EXTERN int T32_QueryAddressObjMmuTranslation (T32_AddressHandle handle, uint16_t translation);
T32EXTERN int T32_QueryAddressObjTargetSizeOfMau(T32_AddressHandle handle);


/** Register Object **/

/* enumeration for all register types */
typedef enum {
	// don't reorder this list - it's used on T32 side, too!
	T32_REGTYPE_NONE,
	T32_REGTYPE_COMMON,     // common register elements
	T32_REGTYPE_R32,        // 32bit register
	T32_REGTYPE_R64,        // 64bit register
	T32_REGTYPE_R128,       // 128bit register
	T32_REGTYPE_R256,       // 256bit register
	T32_REGTYPE_R512,       // 512bit register
	// add new register types here
	T32_REGTYPE_MAX
} T32_RegisterObjType;

/* common register elements */
#define T32_MAX_REGNAME 15
#define T32_INVALID_REGID   ((uint32_t)-1)
typedef struct {
	T32_Object         *next;       // object handling
	T32_RegisterObjType type;       // register type
	uint32_t            id;         // register id
	char                name[T32_MAX_REGNAME+1];    // register name
	uint16_t            core;       // core id to access
} T32_RegisterObj_Common;

/* register type: 32bit register */
typedef struct {
	T32_RegisterObj_Common common;
	uint32_t    value;
} T32_RegisterObj_32bit;

/* register type: 64bit register */
typedef struct {
	T32_RegisterObj_Common common;
	uint64_t    value;
} T32_RegisterObj_64bit;

/* register type: 128bit register */
typedef struct {
	T32_RegisterObj_Common common;
	uint64_t    value[2];
} T32_RegisterObj_128bit;

/* register type: 256bit register */
typedef struct {
	T32_RegisterObj_Common common;
	uint64_t    value[4];
} T32_RegisterObj_256bit;

/* register type: 512bit register */
typedef struct {
	T32_RegisterObj_Common common;
	uint64_t    value[8];
} T32_RegisterObj_512bit;

/* register handle for all register types */
typedef union {
	T32_RegisterObj_Common  common;
	T32_RegisterObj_32bit   r32;
	T32_RegisterObj_64bit   r64;
	T32_RegisterObj_128bit  r128;
	T32_RegisterObj_256bit  r256;
	T32_RegisterObj_512bit  r512;
} T32_RegisterObj;

/* Register handle */
typedef T32_RegisterObj* T32_RegisterHandle;

/* request register object */
T32EXTERN int T32_RequestRegisterObj        (T32_RegisterHandle *pHandle, const T32_RegisterObjType regType);
T32EXTERN int T32_RequestRegisterObjR32     (T32_RegisterHandle *pHandle);
T32EXTERN int T32_RequestRegisterObjR64     (T32_RegisterHandle *pHandle);
T32EXTERN int T32_RequestRegisterObjR128    (T32_RegisterHandle *pHandle);
T32EXTERN int T32_RequestRegisterObjR256    (T32_RegisterHandle *pHandle);
T32EXTERN int T32_RequestRegisterObjR512    (T32_RegisterHandle *pHandle);
T32EXTERN int T32_RequestRegisterObjR32Name (T32_RegisterHandle *pHandle, const char* regName);
T32EXTERN int T32_RequestRegisterObjR64Name (T32_RegisterHandle *pHandle, const char* regName);
T32EXTERN int T32_RequestRegisterObjR128Name(T32_RegisterHandle *pHandle, const char* regName);
T32EXTERN int T32_RequestRegisterObjR256Name(T32_RegisterHandle *pHandle, const char* regName);
T32EXTERN int T32_RequestRegisterObjR512Name(T32_RegisterHandle *pHandle, const char* regName);
T32EXTERN int T32_ReleaseRegisterObj        (T32_RegisterHandle *pHandle);

T32EXTERN int T32_SetRegisterObjName        (T32_RegisterHandle handle, const char* regName);
T32EXTERN int T32_GetRegisterObjName        (T32_RegisterHandle handle, char* regName, uint8_t maxlen);
T32EXTERN int T32_SetRegisterObjId          (T32_RegisterHandle handle, uint32_t regId);
T32EXTERN int T32_GetRegisterObjId          (T32_RegisterHandle handle, uint32_t *pRegId);

T32EXTERN int T32_SetRegisterObjValue32     (T32_RegisterHandle handle, uint32_t value);
T32EXTERN int T32_GetRegisterObjValue32     (T32_RegisterHandle handle, uint32_t *pValue);
T32EXTERN int T32_SetRegisterObjValue64     (T32_RegisterHandle handle, uint64_t value);
T32EXTERN int T32_GetRegisterObjValue64     (T32_RegisterHandle handle, uint64_t *pValue);
T32EXTERN int T32_SetRegisterObjValueArray  (T32_RegisterHandle handle, uint8_t *pArray, uint8_t maxlen);
T32EXTERN int T32_GetRegisterObjValueArray  (T32_RegisterHandle handle, uint8_t *pArray, uint8_t maxlen);

T32EXTERN int T32_SetRegisterObjCore        (T32_RegisterHandle handle, uint16_t core);
T32EXTERN int T32_GetRegisterObjCore        (T32_RegisterHandle handle, uint16_t *pCore);


/** RegisterSet Object **/

#define T32_MAX_REGISTERS 200

typedef struct {
	T32_Object         *next;       // object handling
	int                 nregs;      // number of registers in set
	T32_RegisterHandle  regs[1];    // registers (dynamic array)
} T32_RegisterSetObj;

/* RegisterSet handle */
typedef T32_RegisterSetObj* T32_RegisterSetHandle;

/* request and release register set object */
T32EXTERN int T32_RequestRegisterSetObj     (T32_RegisterSetHandle *pHandle, int numRegisters, T32_RegisterObjType regType);
T32EXTERN int T32_RequestRegisterSetObjR32  (T32_RegisterSetHandle *pHandle, int numRegisters);
T32EXTERN int T32_RequestRegisterSetObjR64  (T32_RegisterSetHandle *pHandle, int numRegisters);
T32EXTERN int T32_ReleaseRegisterSetObj     (T32_RegisterSetHandle *pHandle);

/* access register set properties */
T32EXTERN int T32_SetRegisterSetObjNames    (T32_RegisterSetHandle handle, const char **names, int numNames);
T32EXTERN int T32_SetRegisterSetObjValues32 (T32_RegisterSetHandle handle, const uint32_t *values, int numValues);
T32EXTERN int T32_GetRegisterSetObjValues32 (T32_RegisterSetHandle handle, uint32_t *values, int numValues);

T32EXTERN int T32_ReadRegisterSetObj        (T32_RegisterSetHandle handle);
T32EXTERN int T32_WriteRegisterSetObj       (T32_RegisterSetHandle handle);

/** Symbol Object **/

typedef struct {
	T32_Object          *next;                      // object handling
	char                *name2;                     // symbol name
	char                *path2;                     // symbol path
	T32_AddressHandle   address;                    // symbol address
	uint64_t            size;                       // symbol size
} T32_SymbolObj;

/* Symbol handle */
typedef T32_SymbolObj* T32_SymbolHandle;

/* request symbol object */
T32EXTERN int T32_RequestSymbolObj        (T32_SymbolHandle *pHandle);
T32EXTERN int T32_RequestSymbolObjName    (T32_SymbolHandle *pHandle, const char* symName);
T32EXTERN int T32_RequestSymbolObjAddr    (T32_SymbolHandle *pSymHandle, const T32_AddressHandle addrHandle);
T32EXTERN int T32_ReleaseSymbolObj        (T32_SymbolHandle *pHandle);

/* access object attributes */
T32EXTERN int T32_SetSymbolObjName        (T32_SymbolHandle handle, const char* symName);
T32EXTERN int T32_GetSymbolObjName        (T32_SymbolHandle handle, char* symName, uint8_t maxlen);
T32EXTERN int T32_GetSymbolObjNamePtr     (T32_SymbolHandle handle, char** symName); /* preliminary*/
T32EXTERN int T32_SetSymbolObjPath        (T32_SymbolHandle handle, const char* symPath); /* preliminary*/
T32EXTERN int T32_GetSymbolObjPath        (T32_SymbolHandle handle, char* symPath, uint16_t symPathSize, uint16_t *symPathLen); /* preliminary*/
T32EXTERN int T32_GetSymbolObjPathPtr     (T32_SymbolHandle handle, char** symPath); /* preliminary*/
T32EXTERN int T32_SetSymbolObjAddress     (T32_SymbolHandle symHandle, T32_AddressHandle addrHandle);
T32EXTERN int T32_GetSymbolObjAddress     (T32_SymbolHandle symHandle, T32_AddressHandle* pAddrHandle);
T32EXTERN int T32_GetSymbolObjSize        (T32_SymbolHandle symHandle, uint64_t* pSize);

/** Breakpoint Object **/

typedef struct {
	T32_Object         *next;       // object handling
	T32_AddressHandle   address;    // bp start address
	uint64_t            size;       // bp range size
	uint32_t            types;      // bp types (EMU_breakpoint_bptype_t)
	uint32_t            impl;       // bp implementation (EMU_breakpoint_auxtype_t)
	uint32_t            action;     // bp action (...)
	uint8_t             disabled;   // bp is disabled
	uint32_t            core;       // core id
} T32_BreakpointObj;

#define T32_BP_TYPE_PROGRAM 0x01    // EMU_BREAKSOFT_TYPE_P
#define T32_BP_TYPE_READ    0x02    // EMU_BREAKSOFT_TYPE_R
#define T32_BP_TYPE_WRITE   0x04    // EMU_BREAKSOFT_TYPE_W
#define T32_BP_TYPE_RW      0x06    // EMU_BREAKSOFT_TYPE_RW

#define T32_BP_IMPL_AUTO    0x00    // inactive "AUTO" breakpoint
#define T32_BP_IMPL_SOFT    0x01    // EMU_BREAKSOFT_TYPE_SOFT
#define T32_BP_IMPL_ONCHIP  0x02    // EMU_BREAKSOFT_TYPE_ONCHIP
#define T32_BP_IMPL_HARD    0x04    // EMU_BREAKSOFT_TYPE_HARD
#define T32_BP_IMPL_MARK    0x08    // EMU_BREAKSOFT_TYPE_MARK

#define T32_BP_ACTION_STOP         0x00000001
#define T32_BP_ACTION_SPOT         0x00000002
#define T32_BP_ACTION_ALPHA        0x00000004
#define T32_BP_ACTION_BETA         0x00000008
#define T32_BP_ACTION_CHARLIE      0x00000010
#define T32_BP_ACTION_DELTA        0x00000020
#define T32_BP_ACTION_ECHO         0x00000040
#define T32_BP_ACTION_WATCH        0x02000000
#define T32_BP_ACTION_TRACEENABLE  0x04000000
#define T32_BP_ACTION_TRACEDATA    0x08000000
#define T32_BP_ACTION_TRACEON      0x10000000
#define T32_BP_ACTION_TRACEOFF     0x20000000
#define T32_BP_ACTION_TRACETRIGGER 0x40000000

/* Breakpoint handle */
typedef T32_BreakpointObj* T32_BreakpointHandle;

/* request breakpoint object */
T32EXTERN int T32_RequestBreakpointObj        (T32_BreakpointHandle *pBpHandle);
T32EXTERN int T32_RequestBreakpointObjAddr    (T32_BreakpointHandle *pBpHandle, const T32_AddressHandle addrHandle);
T32EXTERN int T32_ReleaseBreakpointObj        (T32_BreakpointHandle *pBpHandle);

/* breakpoint object access */
T32EXTERN int T32_SetBreakpointObjAddress (T32_BreakpointHandle bpHandle, T32_AddressHandle addrHandle);
T32EXTERN int T32_GetBreakpointObjAddress (T32_BreakpointHandle bpHandle, T32_AddressHandle* pAddrHandle);
T32EXTERN int T32_SetBreakpointObjType    (T32_BreakpointHandle bpHandle, uint32_t   type);
T32EXTERN int T32_GetBreakpointObjType    (T32_BreakpointHandle bpHandle, uint32_t* pType);
T32EXTERN int T32_SetBreakpointObjImpl    (T32_BreakpointHandle bpHandle, uint32_t   impl);
T32EXTERN int T32_GetBreakpointObjImpl    (T32_BreakpointHandle bpHandle, uint32_t* pImpl);
T32EXTERN int T32_SetBreakpointObjAction  (T32_BreakpointHandle bpHandle, uint32_t    act);
T32EXTERN int T32_GetBreakpointObjAction  (T32_BreakpointHandle bpHandle, uint32_t*  pAct);
T32EXTERN int T32_SetBreakpointObjEnable  (T32_BreakpointHandle bpHandle, uint8_t   enable);
T32EXTERN int T32_GetBreakpointObjEnable  (T32_BreakpointHandle bpHandle, uint8_t* pEnable);
T32EXTERN int T32_SetBreakpointObjCore    (T32_BreakpointHandle bpHandle, uint16_t   core);
T32EXTERN int T32_GetBreakpointObjCore    (T32_BreakpointHandle bpHandle, uint16_t* pCore);

/** Memory Bundle Object **/

typedef enum {
	T32_BUFFER_NOTSYNCHED,   // buffer not synchronized with target
	T32_BUFFER_READ,         // buffer was read from target
	T32_BUFFER_WRITTEN,      // buffer war written to target
	T32_BUFFER_ERROR         // error while transferring this buffer
} T32_BufferSynchStatus;

typedef struct {
	T32_BufferSynchStatus   synched;    // buffer synchronised with target?
	T32_AddressHandle       address;    // target address of memory chunks
	T32_BufferHandle        buffer;     // memory data on host
	uint32_t                read;       // memory read or memory write or masked read/write
} T32_MemoryChunk;

typedef struct {
	T32_Object              *next;       // object handling
	uint32_t                 used;       // number of currently used chunks
	uint32_t                 size;       // number of memory chunks in bundle
	T32_MemoryChunk        **chunks;     // memory chunks (dynamic array)
} T32_MemoryBundleObj;

typedef T32_MemoryBundleObj* T32_MemoryBundleHandle;

/* read/write length */
typedef uint32_t T32_Length;
/* bundle index */
typedef uint32_t T32_Index;
/* bundle size */
typedef uint32_t T32_Size;

T32EXTERN int T32_RequestMemoryBundleObj (T32_MemoryBundleHandle *pHandle, const int initial_size);
T32EXTERN int T32_ReleaseMemoryBundleObj (T32_MemoryBundleHandle *pHandle);

T32EXTERN int T32_AddToBundleObjAddrLengthByteArray (T32_MemoryBundleHandle bundleHandle, const T32_AddressHandle addrHandle, const T32_Length length, uint8_t *localbuffer);
T32EXTERN int T32_AddToBundleObjAddrLengthByteArrayMaskArray (T32_MemoryBundleHandle bundleHandle, const T32_AddressHandle addrHandle, const T32_Length length, uint8_t *localbuffer, uint8_t *localmaskbuffer);
T32EXTERN int T32_AddToBundleObjAddrLength (T32_MemoryBundleHandle bundleHandle, const T32_AddressHandle addrHandle, const T32_Length length);

T32EXTERN int T32_GetBundleObjSize (T32_MemoryBundleHandle bundleHandle, T32_Size *size);
T32EXTERN int T32_GetBundleObjSyncStatusByIndex (T32_MemoryBundleHandle bundleHandle, T32_BufferSynchStatus *pSyncStatus, T32_Index index);
T32EXTERN int T32_CopyDataFromBundleObjByIndex (uint8_t* localbuffer, int lbsize, T32_MemoryBundleHandle bundleHandle, T32_Index index);

T32EXTERN int T32_TransferMemoryBundleObj (T32_MemoryBundleHandle bundles);

/** Memory access API **/

T32EXTERN int T32_ReadMemoryObj (T32_BufferHandle bufferHandle, const T32_AddressHandle addressHandle, const T32_Length length);

T32EXTERN int T32_WriteMemoryObj (T32_BufferHandle bufferHandle, const T32_AddressHandle addressHandle, const T32_Length length);

/** Register access API **/

T32EXTERN int T32_ReadRegisterObj (T32_RegisterHandle registerHandle);

T32EXTERN int T32_WriteRegisterObj (T32_RegisterHandle registerHandle);

/** Symbol access API **/

T32EXTERN int T32_QuerySymbolObj (T32_SymbolHandle symbolHandle);             // symbol to query information

/** Breakpoint access API **/

T32EXTERN int T32_WriteBreakpointObj          (T32_BreakpointHandle handle, int set);
T32EXTERN int T32_QueryBreakpointObjCount     (uint32_t* pCount);
T32EXTERN int T32_ReadBreakpointObj           (T32_BreakpointHandle handle);      // by address
T32EXTERN int T32_ReadBreakpointObjByIndex    (T32_BreakpointHandle handle, uint32_t index);

/*******************************/
/*  FDX related API functions  */
/*******************************/

T32EXTERN int  T32_Fdx_Resolve    (const char *Name);
T32EXTERN int  T32_Fdx_Open       (const char *Name, const char *Mode);
T32EXTERN int  T32_Fdx_Close      (int Channel);
T32EXTERN int  T32_Fdx_Receive    (int Channel, void *pData, int Width, int MaxSize);
T32EXTERN int  T32_Fdx_ReceivePoll(int Channel, void *pData, int Width, int MaxSize);
T32EXTERN int  T32_Fdx_Send       (int Channel, void *pData, int Width, int Size);
T32EXTERN int  T32_Fdx_SendPoll   (int Channel, void *pData, int Width, int Size);

/*********************************************/
/*  Direct/TAP Access related API functions  */
/*********************************************/

#define T32_DIRECTACCESS_RELEASE            ((T32_BUNDLEDACCESS_HANDLE)0)
#define T32_DIRECTACCESS_HOLD               ((T32_BUNDLEDACCESS_HANDLE)1)

#define T32_TAPACCESSSEND_HEADERSIZE     6
#define T32_TAPACCESS_MAXBITS            ( (0x3c00 - T32_TAPACCESSSEND_HEADERSIZE) * 8 )
#define T32_TAPACCESS_RELEASE            T32_DIRECTACCESS_RELEASE
#define T32_TAPACCESS_HOLD               T32_DIRECTACCESS_HOLD

#define T32_DIRECTACCESS_INSTANCETYPE_TAP         0
#define T32_DIRECTACCESS_INSTANCETYPE_DAP         1
#define T32_DIRECTACCESS_INSTANCETYPE_AHB         2
#define T32_DIRECTACCESS_INSTANCETYPE_APB         3
#define T32_DIRECTACCESS_INSTANCETYPE_AXI         4
#define T32_DIRECTACCESS_INSTANCETYPE_I2C         5    /* preliminary*/

#define T32_DIRECTACCESS_TRISTATE_UINT32                                                      0
#define T32_DIRECTACCESS_SWD_UNIT32                                                           2
#define T32_DIRECTACCESS_INSTANCE_EXISTS_UNIT32                                               3

#define T32_DIRECTACCESS_XXX_DAP_INSTANCE_UINT32                                            300
#define T32_DIRECTACCESS_XXX_DAPACCESSPORT_UINT32                                           301
#define T32_DIRECTACCESS_XXX_SYSPOWERUPREQ_UINT32                                           302
#define T32_DIRECTACCESS_XXX_BIGENDIAN_UINT32                                               303
#define T32_DIRECTACCESS_XXX_AP_ADDRESS_UINT64                                              304
#define T32_DIRECTACCESS_XXX_AP_BASETYPE_UINT32                                             305
#define T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32                                         T32_DIRECTACCESS_XXX_DAP_INSTANCE_UINT32

#define T32_DIRECTACCESS_TAP_CURRENTINSTANCE_UINT32                                         100
#define T32_DIRECTACCESS_TAP_IRPRE_UINT32                                                   101
#define T32_DIRECTACCESS_TAP_IRPOST_UINT32                                                  102
#define T32_DIRECTACCESS_TAP_DRPRE_UINT32                                                   103
#define T32_DIRECTACCESS_TAP_DRPOST_UINT32                                                  104
#define T32_DIRECTACCESS_TAP_PARKSTATE_UINT32                                               105
#define T32_DIRECTACCESS_TAP_MCTAPSTATE_UINT32                                              106
#define T32_DIRECTACCESS_TAP_MCTCKLEVEL_UINT32                                              107
#define T32_DIRECTACCESS_TAP_DAP_INSTANCE_UINT32                                            T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32
#define T32_DIRECTACCESS_TAP_DAP_ACCESSPORT_UINT32                                          T32_DIRECTACCESS_XXX_DAPACCESSPORT_UINT32
#define T32_DIRECTACCESS_TAP_DAP_JTAGACCESSPORTINDEX_UINT32                                 122
#define T32_DIRECTACCESS_TAP_AUTO_MULTICORETAPSTATE_UINT32                                  123
#define T32_DIRECTACCESS_TAP_SELECT_SHIFT_PATTERN_UINT32                                    124

#define T32_DIRECTACCESS_DAP_TAP_INSTANCE_UINT32                                            200
#define T32_DIRECTACCESS_DAP_SWDP_TARGETSEL_UINT32                                          205
#define T32_DIRECTACCESS_DAP_TIMEOUT_UINT32                                                 206
#define T32_DIRECTACCESS_DAP_SCAN_DAP_OPTION_UINT32                                         208

#define T32_DIRECTACCESS_AHB_DAP_INSTANCE_UINT32                                            T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32
#define T32_DIRECTACCESS_AHB_DAPACCESSPORT_UINT32                                           T32_DIRECTACCESS_XXX_DAPACCESSPORT_UINT32
#define T32_DIRECTACCESS_AHB_SYSPOWERUPREQ_UINT32                                           T32_DIRECTACCESS_XXX_SYSPOWERUPREQ_UINT32
#define T32_DIRECTACCESS_AHB_BIGENDIAN_UINT32                                               T32_DIRECTACCESS_XXX_BIGENDIAN_UINT32
#define T32_DIRECTACCESS_AHB_AP_ADDRESS_UINT64                                              T32_DIRECTACCESS_XXX_AP_ADDRESS_UINT64 /*preliminary*/
#define T32_DIRECTACCESS_AHB_AP_BASETYPE_UINT32                                             T32_DIRECTACCESS_XXX_AP_BASETYPE_UINT32 /*preliminary*/
#define T32_DIRECTACCESS_AHB_AP_BASEINSTANCE_UINT32                                         T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32 /*preliminary*/
#define T32_DIRECTACCESS_AHB_CORTEXM_UINT32                                                 310

#define T32_DIRECTACCESS_APB_DAP_INSTANCE_UINT32                                            T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32
#define T32_DIRECTACCESS_APB_DAPACCESSPORT_UINT32                                           T32_DIRECTACCESS_XXX_DAPACCESSPORT_UINT32
#define T32_DIRECTACCESS_APB_SYSPOWERUPREQ_UINT32                                           T32_DIRECTACCESS_XXX_SYSPOWERUPREQ_UINT32
#define T32_DIRECTACCESS_APB_BIGENDIAN_UINT32                                               T32_DIRECTACCESS_XXX_BIGENDIAN_UINT32
#define T32_DIRECTACCESS_APB_AP_ADDRESS_UINT64                                              T32_DIRECTACCESS_XXX_AP_ADDRESS_UINT64 /*preliminary*/
#define T32_DIRECTACCESS_APB_AP_BASETYPE_UINT32                                             T32_DIRECTACCESS_XXX_AP_BASETYPE_UINT32 /*preliminary*/
#define T32_DIRECTACCESS_APB_AP_BASEINSTANCE_UINT32                                         T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32 /*preliminary*/

#define T32_DIRECTACCESS_AXI_DAP_INSTANCE_UINT32                                            T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32
#define T32_DIRECTACCESS_AXI_DAPACCESSPORT_UINT32                                           T32_DIRECTACCESS_XXX_DAPACCESSPORT_UINT32
#define T32_DIRECTACCESS_AXI_SYSPOWERUPREQ_UINT32                                           T32_DIRECTACCESS_XXX_SYSPOWERUPREQ_UINT32
#define T32_DIRECTACCESS_AXI_BIGENDIAN_UINT32                                               T32_DIRECTACCESS_XXX_BIGENDIAN_UINT32
#define T32_DIRECTACCESS_AXI_AP_ADDRESS_UINT64                                              T32_DIRECTACCESS_XXX_AP_ADDRESS_UINT64 /*preliminary*/
#define T32_DIRECTACCESS_AXI_AP_BASETYPE_UINT32                                             T32_DIRECTACCESS_XXX_AP_BASETYPE_UINT32 /*preliminary*/
#define T32_DIRECTACCESS_AXI_AP_BASEINSTANCE_UINT32                                         T32_DIRECTACCESS_XXX_AP_BASEINSTANCE_UINT32 /*preliminary*/


#define T32_SCAN_DAP_OPTION_OUTBUFFER_IMMEDIATELY                                           0x0
#define T32_SCAN_DAP_OPTION_OUTBUFFER_DELAYED                                               0x1
#define T32_SCAN_DAP_OPTION_OUTBUFFER_DEPEND_TO_CONFIG                                      0x2

#define T32_DIRECTACCESS_TIMESTAMPFORMAT_T32                                                0x0 /*preliminary*/
#define T32_DIRECTACCESS_TIMESTAMPFORMAT_NS                                                 0x1 /*preliminary*/
#define T32_DIRECTACCESS_TIMESTAMPFORMAT_US                                                 0x2 /*preliminary*/
#define T32_DIRECTACCESS_TIMESTAMPFORMAT_PS                                                 0x3 /*preliminary*/
#define T32_DIRECTACCESS_TIMESTAMPFORMAT_OPT_RAW                                     0x80000000 /*preliminary*/

#define T32_DAPACCESS_REGISTERSET_DP     0
#define T32_DAPACCESS_REGISTERSET_AP     1
#define T32_DAPACCESS_REGISTERSET_IDCODE 2
#define T32_DAPACCESS_REGISTERSET_ABORT  3

#define T32_DAPACCESS_RW_READ       0
#define T32_DAPACCESS_RW_READWRITE  1
#define T32_DAPACCESS_RW_WRITE      2

#define T32_DAPAPACCESS_RW_READ  0
#define T32_DAPAPACCESS_RW_WRITE 1

#define T32_TAPSTATE_RUN_TEST_IDLE   12
#define T32_TAPSTATE_SELECT_DR_SCAN  7

#define T32_TAPACCESS_SET(x)             (2|((x)&1))
#define T32_TAPACCESS_SET_LOW            2
#define T32_TAPACCESS_SET_HIGH           3
#define T32_TAPACCESS_SET_0              T32_TAPACCESS_SET_LOW
#define T32_TAPACCESS_SET_1              T32_TAPACCESS_SET_HIGH
#define T32_TAPACCESS_TDO                0x00
#define T32_TAPACCESS_TDI                0x04
#define T32_TAPACCESS_TMS                0x08
#define T32_TAPACCESS_TCK                0x0c
#define T32_TAPACCESS_nTRST              0x10
#define T32_TAPACCESS_nRESET             0x14
#define T32_TAPACCESS_nRESET_LATCH       0x18
#define T32_TAPACCESS_VTREF              0x1c
#define T32_TAPACCESS_VTREF_LATCH        0x20
#define T32_TAPACCESS_nENOUT             0x24
#define T32_TAPACCESS_SHIFT              0x30
#define T32_TAPACCESS_SHIFTTMS           0x40
#define T32_TAPACCESS_SHIFTIR            0x50
#define T32_TAPACCESS_SHIFTDR            0x60
#define T32_TAPACCESS_SLEEP_MS           0x7c
#define T32_TAPACCESS_SLEEP              T32_TAPACCESS_SLEEP_MS
#define T32_TAPACCESS_SLEEP_US           0x7d
#define T32_TAPACCESS_SLEEP_HALF_CLOCK   0x7e
#define T32_TAPACCESS_SPECIAL            0x80

#define T32_USERSIGNALEVENT 67

#define T32_DIRECTACCESS_USERSIGNAL_SET_ON                                 ((uint32_t)0x10000000)
#define T32_DIRECTACCESS_USERSIGNAL_SET_OFF                                ((uint32_t)0x20000000)
#define T32_DIRECTACCESS_USERSIGNAL_GET                                    ((uint32_t)0x30000000)
#define T32_DIRECTACCESS_USERSIGNAL_WAS_TRIGGER                            ((uint32_t)0x40000000)
#define T32_DIRECTACCESS_USERSIGNAL_ENABLE_EVENT_RISING                    ((uint32_t)0x50000000)
#define T32_DIRECTACCESS_USERSIGNAL_ENABLE_EVENT_FALLING                   ((uint32_t)0x60000000)
#define T32_DIRECTACCESS_USERSIGNAL_DISABLE_EVENT_RISING                   ((uint32_t)0x80000000)
#define T32_DIRECTACCESS_USERSIGNAL_DISABLE_EVENT_FALLING                  ((uint32_t)0x90000000)
#define T32_DIRECTACCESS_USERSIGNAL_GET_NEXT_EVENT                         ((uint32_t)0xB0000000)
#define T32_DIRECTACCESS_USERSIGNAL_GET_EVENT_COUNT                        ((uint32_t)0xC0000000)
#define T32_DIRECTACCESS_USERSIGNAL_CLEAR_EVENT                            ((uint32_t)0xD0000000)
#define T32_DIRECTACCESS_USERSIGNAL_CLEAR_ALL_EVENTS                       ((uint32_t)0xE0000000)
#define T32_DIRECTACCESS_USERSIGNAL_GET_LAST_ERROR                         ((uint32_t)0xF0000000)
#define T32_DIRECTACCESS_USERSIGNAL_POLL_EVENTS                            ((uint32_t)0x11000000)
#define T32_DIRECTACCESS_USERSIGNAL_SET_DIRECTION_FROM_TARGET              ((uint32_t)0x21000000)
#define T32_DIRECTACCESS_USERSIGNAL_SET_DIRECTION_TO_TARGET                ((uint32_t)0x31000000)
#define T32_DIRECTACCESS_USERSIGNAL_SET_TRIGGER_POLARITY_LOW               ((uint32_t)0x41000000)
#define T32_DIRECTACCESS_USERSIGNAL_SET_TRIGGER_POLARITY_HIGH              ((uint32_t)0x51000000)
#define T32_DIRECTACCESS_USERSIGNAL_DISABLE_AND_CLEAR_ALL_EVENTS           ((uint32_t)0x61000000)
#define T32_DIRECTACCESS_USERSIGNAL_ENABLE_EVENT_CHANGE                    ((uint32_t)0x71000000)
#define T32_DIRECTACCESS_USERSIGNAL_DISABLE_EVENT_CHANGE                   ((uint32_t)0x81000000)
#define T32_DIRECTACCESS_USERSIGNAL_CONNECT_CROSSTRIGGER_ASSERT_ON_ASSERT  ((uint32_t)0xB1000000)
#define T32_DIRECTACCESS_USERSIGNAL_DISCONNECT_CROSSTRIGGER                ((uint32_t)0xC1000000)
#define T32_DIRECTACCESS_USERSIGNAL_CLEAR_CROSSTRIGGER                     ((uint32_t)0xD1000000)

#define T32_DIRECTACCESS_USERSIGNAL_RESULT_NA   ((uint32_t)0x0000000A)
#define T32_DIRECTACCESS_USERSIGNAL_RESULT_ON   ((uint32_t)0x00000001)
#define T32_DIRECTACCESS_USERSIGNAL_RESULT_OFF  ((uint32_t)0x00000000)

#define T32_DIRECTACCESS_USERSIGNAL_POWER       ((uint32_t)0x00000001)
#define T32_DIRECTACCESS_USERSIGNAL_RESET       ((uint32_t)0x00000002)
#define T32_DIRECTACCESS_USERSIGNAL_ENOUT       ((uint32_t)0x00000003)
#define T32_DIRECTACCESS_USERSIGNAL_TRIGGER     ((uint32_t)0x00000004)

#define T32_DIRECTACCESS_USERSIGNAL_JTAG_TDI    ((uint32_t)0x00000010)
#define T32_DIRECTACCESS_USERSIGNAL_JTAG_TMS    ((uint32_t)0x00000011)
#define T32_DIRECTACCESS_USERSIGNAL_JTAG_TCK    ((uint32_t)0x00000012)
#define T32_DIRECTACCESS_USERSIGNAL_JTAG_TDO    ((uint32_t)0x00000013)
#define T32_DIRECTACCESS_USERSIGNAL_JTAG_RTCK   ((uint32_t)0x00000014)
#define T32_DIRECTACCESS_USERSIGNAL_JTAG_TRST   ((uint32_t)0x00000015)

#define SHIFTRAW_OPTION_NONE             0x0000
#define SHIFTRAW_OPTION_LASTTMS_ONE      0x0008
#define SHIFTRAW_OPTION_TMS_ZERO         SHIFTRAW_OPTION_NONE
#define SHIFTRAW_OPTION_TMS_ONE          0x0010
#define SHIFTRAW_OPTION_TDI_ZERO         SHIFTRAW_OPTION_NONE
#define SHIFTRAW_OPTION_TDI_LASTTDO      0x0020
#define SHIFTRAW_OPTION_TDI_ONE          0x0040

#define T32_I2CACCESS_ACTION_WRITE           0  /* preliminary*/
#define T32_I2CACCESS_ACTION_READ            1  /* preliminary*/
#define T32_I2CACCESS_ACTION_READ_NONAK      2  /* preliminary*/

#define T32_I2CACCESS_RESULT_OK            0  /* preliminary*/
#define T32_I2CACCESS_RESULT_DEVIDNAK      2  /* preliminary*/
#define T32_I2CACCESS_RESULT_CMDNAK        3  /* preliminary*/
#define T32_I2CACCESS_RESULT_DATANAK       4  /* preliminary*/
#define T32_I2CACCESS_RESULT_SDAZERO       5  /* preliminary*/
#define T32_I2CACCESS_RESULT_SCLZERO       6  /* preliminary*/
#define T32_I2CACCESS_RESULT_NODRIVER      7  /* preliminary*/
#define T32_I2CACCESS_RESULT_TIMEOUT       8  /* preliminary*/
#define T32_I2CACCESS_RESULT_INVALIDPORT   9  /* preliminary */

#define T32_I2C_RAWACC_SEND_START      0x00000100  /* preliminary*/
#define T32_I2C_RAWACC_SEND_STOP       0x00000200  /* preliminary*/
#define T32_I2C_RAWACC_SEND_ACK        0x00000400  /* preliminary*/
#define T32_I2C_RAWACC_SEND_IGNNAK     0x00000800  /* preliminary*/
#define T32_I2C_RAWACC_RECV_ACK        0x00000100  /* preliminary*/
#define T32_I2C_RAWACC_RECV_ERR        0x00000200  /* preliminary*/
#define T32_I2C_RAWACC_RECVERR_NOACK   0x00010000  /* preliminary*/
#define T32_I2C_RAWACC_RECVERR_ARB     0x00020000  /* preliminary*/
#define T32_I2C_RAWACC_RECVERR_TIMEOUT 0x00030000  /* preliminary*/

#define T32_TAPAccessRelease T32_DirectAccessRelease
#define T32_TAPAccessAlloc T32_BundledAccessAlloc
#define T32_TAPAccessExecute T32_BundledAccessExecute
#define T32_TAPAccessFree T32_BundledAccessFree
#define T32_TAPACCESS_HANDLE T32_BUNDLEDACCESS_HANDLE

typedef union {
	uint32_t uint32;
	uint64_t uint64;
} T32_Param;

typedef void* T32_BUNDLEDACCESS_HANDLE;

T32EXTERN T32_Param T32_ParamFromUint32(uint32_t value);
T32EXTERN T32_Param T32_ParamFromUint64(uint64_t value);
T32EXTERN T32_BUNDLEDACCESS_HANDLE T32_BundledAccessAlloc(void);
T32EXTERN int  T32_BundledAccessExecute(T32_BUNDLEDACCESS_HANDLE Handle, T32_BUNDLEDACCESS_HANDLE SequenceHandle);
T32EXTERN int  T32_BundledAccessFree(T32_BUNDLEDACCESS_HANDLE Handle);
T32EXTERN int  T32_DirectAccessRelease(void);
T32EXTERN int  T32_DirectAccessResetAll(T32_BUNDLEDACCESS_HANDLE Handle);
T32EXTERN int  T32_DirectAccessSetInfo(T32_BUNDLEDACCESS_HANDLE Handle, int nInstanceType, unsigned int nInstance, int nInfoID, T32_Param value);
T32EXTERN int  T32_DirectAccessGetInfo(T32_BUNDLEDACCESS_HANDLE Handle, int nInstanceType, unsigned int nInstance, int nInfoID, T32_Param *value);
T32EXTERN int  T32_DirectAccessGetTimestamp(T32_BUNDLEDACCESS_HANDLE Handle, int Format, uint64_t *result); /* preliminary*/
T32EXTERN int  T32_TAPAccessSetInfo(int irpre, int irpost, int drpre, int drpost, int tristate, int tapstate, int tcklevel, int reserved);
T32EXTERN int  T32_TAPAccessSetInfo2(T32_BUNDLEDACCESS_HANDLE Handle, int irpre, int irpost, int drpre, int drpost, int tristate, int tapstate, int tcklevel, int reserved);
T32EXTERN int  T32_TAPAccessShiftRaw(T32_BUNDLEDACCESS_HANDLE Handle, int NumberOfBits, const uint8_t *pTMSBits,  const uint8_t *pTDIBits, uint8_t *pTDOBits, int Options);
T32EXTERN int  T32_TAPAccessShiftIR(T32_BUNDLEDACCESS_HANDLE Handle, int NumberOfBits, const uint8_t *pOutBits,  uint8_t *pInBits);
T32EXTERN int  T32_TAPAccessShiftDR(T32_BUNDLEDACCESS_HANDLE Handle, int NumberOfBits, const uint8_t *pOutBits,  uint8_t *pInBits);
T32EXTERN int  T32_TAPAccessJTAGResetWithTMS(T32_BUNDLEDACCESS_HANDLE Handle, unsigned int nTapInstance);
T32EXTERN int  T32_TAPAccessJTAGResetWithTRST(T32_BUNDLEDACCESS_HANDLE Handle, unsigned int nTapInstance, int32_t nTRSTAssertTimeUS, int32_t nDelayAfterTRSTReleaseUS);
T32EXTERN int  T32_TAPAccessSetShiftPattern(T32_BUNDLEDACCESS_HANDLE Handle, unsigned int nTapInstance, uint32_t nReturnIRCount, uint32_t nReturnDRCount, uint32_t nGotoIRCount, uint32_t nGotoDRCount, uint32_t nReturnIR, uint32_t nReturnDR, uint32_t nGotoIR, uint32_t nGotoDR, unsigned int nPattern);
T32EXTERN int  T32_TAPAccessDirect(T32_BUNDLEDACCESS_HANDLE Handle, int NumberOfBytes, const uint8_t *pOutBytes, uint8_t *pInBytes);
T32EXTERN int  T32_DirectAccessUserSignal(T32_BUNDLEDACCESS_HANDLE Handle, int NumberOfAccesses, const uint32_t *pOutAccesses, uint32_t *pInAccesses);
T32EXTERN int  T32_DAPAccessScan(T32_BUNDLEDACCESS_HANDLE Handle, unsigned int nDapInstance, int nRegisterSet, int nRW, uint32_t nRegisterAddress, uint32_t nDataIn, uint32_t *pDataOut);
T32EXTERN int  T32_DAPAccessInitSWD(T32_BUNDLEDACCESS_HANDLE Handle, unsigned int nDapInstance);
T32EXTERN int  T32_DAPAPAccessReadWrite(T32_BUNDLEDACCESS_HANDLE Handle, int nAPType, unsigned int nInstance, int nRW, uint64_t nAddress, uint8_t * pData, unsigned int nByteWidth, unsigned int nByteSize, int bNonIncrement, uint32_t nFlags);
T32EXTERN int  T32_I2CAccess(T32_BUNDLEDACCESS_HANDLE Handle, unsigned int nInstance, uint32_t nAction, uint32_t nDeviceID, const uint8_t* pCommandData, unsigned int nCommandByteLength, uint8_t* pData, unsigned int nDataByteLength); /* preliminary*/
T32EXTERN int  T32_I2CRawAccess(T32_BUNDLEDACCESS_HANDLE Handle, unsigned int nInstance, const uint32_t *send, uint32_t *recv, int len);
T32EXTERN int  T32_DirectAccessExecuteLua(T32_BUNDLEDACCESS_HANDLE Handle, const char* filename, int mode, const uint8_t* inputBuf, int inputBuflen, uint8_t* outputBuf, int outputBufLen); /* preliminary*/

/*********************************************/
/*  Lua Script related API functions         */
/*********************************************/
T32EXTERN int  T32_ExecuteLua(const char* filename, int mode, const uint8_t* inputBuf, int inputBuflen, uint8_t* outputBuf, int outputBufLen); /* preliminary*/

#ifdef T32INTERNAL_MAGIC
#if T32INTERNAL_MAGIC == 0xfe8ac993
// These definitions are not intended to be used by a customer.
// So these definitions are protected by the above #if
typedef struct Channel_s {
	const struct T32InternalLineDriver *pLineDriver;
	void *pLineParams;
} Channel_t;
typedef struct LineStruct_s LineStruct;
struct T32InternalLineDriver {
	int      (*Config)(char *in);                              // LINE_LineConfig(char *in);
	int      (*Init)(char *message);                           // LINE_LineInit(char *message);
	void     (*Exit)(void);                                    // LINE_LineExit(void);
	int      (*GetSocket)(void);                               // LINE_LineDriverGetSocket(void);
	int      (*Transmit)(unsigned char *in, int size);         // LINE_LineTransmit(unsigned char *in, int size);
	int      (*Receive)(unsigned char *out);                   // LINE_LineReceive(unsigned char *out);
	int      (*ReceiveNotifyMessage)(unsigned char *package);  // LINE_ReceiveNotifyMessage(unsigned char *package);
	int      (*Sync)(void);                                    // LINE_LineSync(void);
	int      (*GetParamsSize)(void);                           // LINE_GetLineParamsSize(void);
	void     (*DefaultParams)(LineStruct * ParametersOut);     // LINE_DefaultLineParams(LineStruct * ParametersOut);
	void     (*SetParams)(LineStruct * params);                // LINE_SetLine(LineStruct * params);
	void     (*SetReceiveToggleBit)(int value);                // LINE_SetReceiveToggleBit(int value);
	int      (*GetReceiveToggleBit)(void);                     // LINE_GetReceiveToggleBit(void);
	unsigned char (*GetNextMessageId)(void);                   // LINE_GetNextMessageId(void);
	unsigned char (*GetMessageId)(void);                       // LINE_GetMessageId(void);
	int      (*NotificationPending)(void);                     // LINE_NotificationPending(void);
};
//extern T32_THREADLOCAL struct Channel_s *gChannel;
extern const struct T32InternalLineDriver *gT32InternalLineDriverNetAssist;
extern const struct T32InternalLineDriver *gT32InternalLineDriverNetTcp;
#endif
#endif

# ifdef __cplusplus
}
# endif
#endif /* !_NO_PROTO */

#define LINE_MSIZE                          (16384+256)

#ifdef _WIN32
# ifndef T32HOST_WIN
#  define T32HOST_WIN
# endif
# ifndef T32HOST_LE
#  define T32HOST_LE
# endif
# if !defined(T32HOST_WIN_X86) && (defined(_M_IX86) || defined(__i386__))
#  define T32HOST_WIN_X86
# endif
# if !defined(T32HOST_WIN_X64) && (defined(_M_X64) || defined(__x86_64__))
#  define T32HOST_WIN_X64
# endif
# if !defined(T32HOST_ILP32) && defined(T32HOST_WIN_X86)
#  define T32HOST_ILP32
# endif
# if !defined(T32HOST_LLP64) && defined(T32HOST_WIN_X64)
#  define T32HOST_LLP64
# endif
#endif

#if defined(__linux__) || defined(__CYGWIN__)
# ifndef T32HOST_LINUX
#  define T32HOST_LINUX
# endif
# ifndef T32HOST_UNIX
#  define T32HOST_UNIX
# endif
# if !defined(T32HOST_LINUX_X86) && defined(__i386__)
#  define T32HOST_LINUX_X86
# endif
# if !defined(T32HOST_LINUX_X64) && defined(__x86_64__)
#  define T32HOST_LINUX_X64
# endif
# if !defined(T32HOST_LINUX_PPC) && defined(__ppc__)
#  define T32HOST_LINUX_PPC
# endif
# if !defined(T32HOST_LINUX_ARM) && defined(__SOFTFP__) && defined(__ARM_ARCH_4T__)
#  define T32HOST_LINUX_ARM
# endif
# if !defined(T32HOST_LINUX_ARMHF) && !defined(__SOFTFP__) && defined(__ARM_ARCH_7A__)
#  define T32HOST_LINUX_ARMHF
# endif
# if !defined(T32HOST_LINUX_ARM64) && defined(__aarch64__)
#  define T32HOST_LINUX_ARM64
# endif
# if !defined(T32HOST_ILP32) && !defined(__LP64__)
#  define T32HOST_ILP32
# endif
# if !defined(T32HOST_LP64) && defined(__LP64__)
#  define T32HOST_LP64
# endif
# if !defined(T32HOST_LE) && (defined(T32HOST_LINUX_X86) || defined(T32HOST_LINUX_X64) || defined(T32HOST_LINUX_ARM) || defined(T32HOST_LINUX_ARMHF) || defined(T32HOST_LINUX_ARM64))
#  define T32HOST_LE
# endif
# if !defined(T32HOST_BE) && defined(T32HOST_LINUX_PPC)
#  define T32HOST_BE
# endif
#endif

#if defined(__APPLE__)
# ifndef T32HOST_MACOSX
#  define T32HOST_MACOSX
# endif
# ifndef T32HOST_UNIX
#  define T32HOST_UNIX
# endif
# ifndef T32HOST_LE
#  define T32HOST_LE
# endif
# if !defined(T32HOST_MACOSX_X64) && defined(__x86_64__)
#  define T32HOST_MACOSX_X64
# endif
# if !defined(T32HOST_MACOSX_ARM64) && defined(__arm64__)
#  define T32HOST_MACOSX_ARM64
# endif
# if !defined(T32HOST_ILP32) && !defined(__LP64__)
#  define T32HOST_ILP32
# endif
# if !defined(T32HOST_LP64) && defined(__LP64__)
#  define T32HOST_LP64
# endif
#endif

#if defined(__sun)
# ifndef T32HOST_SOL
#  define T32HOST_SOL
# endif
# ifndef T32HOST_UNIX
#  define T32HOST_UNIX
# endif
# if !defined(T32HOST_SOL_SPARC) && (defined(__sparc__) || defined(__sparc))
#  define T32HOST_SOL_SPARC
# endif
# if !defined(T32HOST_ILP32) && defined(T32HOST_SOL_SPARC)
#  define T32HOST_ILP32
# endif
# if !defined(T32HOST_BE) && defined(T32HOST_SOL_SPARC)
#  define T32HOST_BE
# endif
#endif

#if defined(T32HOST_WIN) && defined(_MSC_VER)
# define SETWORDVAR(A,B) ((*((unsigned short *)&(A)))=(*((unsigned short *)&(B))))
# define SETLONGVAR(A,B) ((*((int *)&(A)))=(*((int *)&(B))))
# define SETQUADVAR(A,B) ((*((uint64_t *)&(A)))=(*((uint64_t *)&(B))))
#endif

#if defined(T32HOST_LE) && !defined(SETWORDVAR)
# define SETWORDVAR(A,B) (((unsigned char *)&(A))[0]=((unsigned char *)&(B))[0],((unsigned char *)&(A))[1]=((unsigned char *)&(B))[1])
# define SETLONGVAR(A,B) (((unsigned char *)&(A))[0]=((unsigned char *)&(B))[0],((unsigned char *)&(A))[1]=((unsigned char *)&(B))[1],((unsigned char *)&(A))[2]=((unsigned char *)&(B))[2],((unsigned char *)&(A))[3]=((unsigned char *)&(B))[3])
# define SETQUADVAR(A,B) ( \
	((unsigned char *)&(A))[0]=((unsigned char *)&(B))[0],((unsigned char *)&(A))[1]=((unsigned char *)&(B))[1],((unsigned char *)&(A))[2]=((unsigned char *)&(B))[2],((unsigned char *)&(A))[3]=((unsigned char *)&(B))[3], \
	((unsigned char *)&(A))[4]=((unsigned char *)&(B))[4],((unsigned char *)&(A))[5]=((unsigned char *)&(B))[5],((unsigned char *)&(A))[6]=((unsigned char *)&(B))[6],((unsigned char *)&(A))[7]=((unsigned char *)&(B))[7])
#endif

#if defined(T32HOST_BE)
# define SETWORDVAR(A,B) (((unsigned char *)&(A))[0]=((unsigned char *)&(B))[1],((unsigned char *)&(A))[1]=((unsigned char *)&(B))[0])
# define SETLONGVAR(A,B) (((unsigned char *)&(A))[0]=((unsigned char *)&(B))[3],((unsigned char *)&(A))[1]=((unsigned char *)&(B))[2],((unsigned char *)&(A))[2]=((unsigned char *)&(B))[1],((unsigned char *)&(A))[3]=((unsigned char *)&(B))[0])
# define SETQUADVAR(A,B) ( \
	((unsigned char *)&(A))[0]=((unsigned char *)&(B))[7],((unsigned char *)&(A))[1]=((unsigned char *)&(B))[6],((unsigned char *)&(A))[2]=((unsigned char *)&(B))[5],((unsigned char *)&(A))[3]=((unsigned char *)&(B))[4], \
	((unsigned char *)&(A))[4]=((unsigned char *)&(B))[3],((unsigned char *)&(A))[5]=((unsigned char *)&(B))[2],((unsigned char *)&(A))[6]=((unsigned char *)&(B))[1],((unsigned char *)&(A))[7]=((unsigned char *)&(B))[0])
#endif

#endif /* T32_H */

